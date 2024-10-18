import os
import subprocess
from graphviz import Digraph
import networkx as nx
from collections import defaultdict
import re
from tarjan import tarjan_scc, COLORS

def clean_string(s):
    return s.replace(':', '_').replace('\t', ' ').replace('\\', '').replace('"', '').replace("'", "")

def extract_tags_from_ctags(file):
    functions = {}
    variables = set()

    with open(file, 'r') as f:
        for line in f:
            if not line.startswith('!'):
                parts = line.split('\t')
                name, kind, file = clean_string(parts[0]), parts[3], parts[1]
                if kind == 'f':
                    functions[name] = file
                elif kind == 'v':
                    variables.add(name)

    return variables, functions

def find_references(item, source_files):
    references = []
    for src_file in source_files:
        # Modify the grep command to search for both "item" and "*item"
        cmd = f"grep -n '\\b{item}\\b\\|\\b\*{item}\\b' {src_file}"
        try:
            result = subprocess.check_output(cmd, shell=True).decode('utf-8').splitlines()
            references.extend([(src_file, line) for line in result])
        except subprocess.CalledProcessError:
            continue
    return references


def build_dependency(variables, functions, source_files):
    dependencies = {var: set() for var in variables}  # Initialize with all variables with an empty set
    
    for variable in variables:
        refs = find_references(variable, source_files)
        for file, line in refs:
            for other_variable in variables:
                if other_variable != variable and other_variable in line:
                    dependencies[variable].add(other_variable)
    
    func_var_dependencies = {}
    for function in functions:
        refs = find_references(function, source_files)
        for file, line in refs:
            for variable in variables:
                if variable in line:
                    func_var_dependencies.setdefault(function, set()).add(variable)

        # Debug output to check if progname is referenced
        if 'progname' in func_var_dependencies.get(function, {}):
            print(f"Function {function} references progname in line: {line}")

    return dependencies, func_var_dependencies

def create_dependency_graphs(variable_dependencies, function_variable_dependencies, functions):
    var_dot = Digraph('VariableDependencies', graph_attr={'rankdir': 'LR', 'size': '12,12'})

    # 添加所有的变量到图中，即使它们并没有被引用或依赖
    for variable in variable_dependencies.keys():
        var_dot.node(variable, shape="ellipse", color="blue")

    for variable, depends_on in variable_dependencies.items():
        for depend in depends_on:
            var_dot.edge(variable, depend)


    func_var_dot = Digraph('FunctionVariableDependencies', graph_attr={'rankdir': 'LR', 'size': '12,12'})
    for function, variables in function_variable_dependencies.items():
        if functions[function].endswith('.c'):
            func_var_dot.node(function, color="green")  # .c functions in green
        else:
            func_var_dot.node(function, color="red")  # library functions in red
        for variable in variables:
            func_var_dot.node(variable, shape="ellipse", color="blue")  # global variables in blue
            func_var_dot.edge(function, variable)
    
    # Add isolated variables
    all_variables_in_graph = {variable for _, variables in function_variable_dependencies.items() for variable in variables}
    isolated_variables = set(variables) - all_variables_in_graph
    for variable in isolated_variables:
        func_var_dot.node(variable, shape="ellipse", color="blue")  # Isolated global variables in blue

    return var_dot, func_var_dot

def add_scc_to_graph(dot, adj_list, root_sccs):
    sccs = tarjan_scc(adj_list)
    for scc in sccs:
        # Define a unique border color for each SCC
        border_color = COLORS[sccs.index(scc) % len(COLORS)]

        if scc in root_sccs:
            # Background color for root SCCs
            fill_color = "yellow"
        else:
            # No background color for non-root SCCs
            fill_color = "white"
        
        for variable in scc:
            dot.node(variable, shape="ellipse", color=border_color, style="filled", fillcolor=fill_color)


            

def create_function_body_graph(func_body_dependencies, functions):
    func_body_dot = Digraph('FunctionBodyDependencies', graph_attr={'rankdir': 'TB', 'size': '12,12'}, engine='circo')

    for function, variables in func_body_dependencies.items():
        if functions[function].endswith('.c'):
            func_body_dot.node(function, color="green")  # .c functions in green
        else:
            func_body_dot.node(function, color="red")  # library functions in red

        for variable in variables:
            func_body_dot.node(variable, shape="ellipse", color="blue")  # global variables in blue
            func_body_dot.edge(function, variable)

    return func_body_dot

def export_function_body_to_gexf(func_body_dependencies, file_name):
    G = nx.DiGraph()

    for function, variables in func_body_dependencies.items():
        G.add_node(function, type="function", color="green")  # .c functions in green
        for variable in variables:
            G.add_node(variable, type="variable", color="blue")  # global variables in blue
            G.add_edge(function, variable)

    nx.write_gexf(G, file_name)
    
    
def write_variable_usage_to_file(func_body_dependencies, file_name="variable_usage.txt"):
    variable_usage = defaultdict(lambda: defaultdict(int))

    for func, vars_used in func_body_dependencies.items():
        for var in vars_used:
            variable_usage[var][func] += 1

    with open(file_name, 'w') as f:
        for var, funcs in variable_usage.items():
            total_usage = sum(funcs.values())
            f.write(f"{var} (used {total_usage} times):\n")
            for func, count in funcs.items():
                f.write(f"    {func}: {count} times\n")
            f.write("\n")
            
def find_roots_from_sccs(sccs, adj_list):
    all_scc_targets = []
    for scc in sccs:
        targets = []
        for node in scc:
            targets.extend([n for n in adj_list.get(node, []) if n not in scc])
        all_scc_targets.extend(targets)

    all_scc_targets = set(all_scc_targets)
    root_sccs = [scc for scc in sccs if not any(node in all_scc_targets for node in scc)]
    return root_sccs
