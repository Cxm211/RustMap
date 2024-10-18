import os
from tree_sitter import Language, Parser
import graphviz
import time
from graphviz import Digraph
import sys
sys.setrecursionlimit(5000)  # 设置为5000或更高
from GlobDep_FnDep import extract_tags_from_ctags,build_dependency
SO_FILE_PATH = '/root/rustmap/tree-sitter-c.so'
C_LANGUAGE = Language(SO_FILE_PATH, 'c')
parser = Parser()
parser.set_language(C_LANGUAGE)
total_get_node_source_calls = 0
fn_get_node_source_calls = {}


def has_descendant_of_type(node, target_type):
    """递归查找是否存在特定类型的后代节点"""
    if node.type == target_type:
        return True
    for child in node.children:
        if has_descendant_of_type(child, target_type):
            return True
    return False


def get_node_source(node, code):
    return code[node.start_byte:node.end_byte]

def extract_global_variables_from_ctags(file):
    global_vars = {}
    static_global_vars = {}
    extern_vars = {}
    
    with open(file, 'r') as f:
        for line in f:
            if not line.startswith('!'):
                parts = line.split('\t')
                name, kind, src_file, file_scope = parts[0], parts[3], parts[1], "file:" in parts[-1]
                if kind == 'v':
                    if file_scope: # 🔥特别注意👇
                        static_global_vars[name+'-'+src_file] = src_file
                    else:
                        global_vars[name] = src_file
                if kind == 'x':
                    extern_vars[name] = src_file
    # print("🆕",global_vars)
    # time.sleep(4)
    # print("💊",static_global_vars)
    # time.sleep(4)
    return global_vars, static_global_vars, extern_vars



def create_treesitter_graph(node, source_code, graph=None, parent=None, counter={'i': 0}, format='svg'):
    counter['i'] += 1
    curr_node = 'node'+str(counter['i'])
    if graph is None:
        graph = graphviz.Digraph()
        graph.attr(rankdir='LR')  # 设置从左到右的方向
        graph.attr(nodesep='0.75')  # 设置节点间的间距
        graph.attr(ranksep='2')  # 设置不同层级节点之间的间距
    label = node.type
    if node.is_named:
        label += f" ({node.start_point} - {node.end_point})"
    graph.node(curr_node, label=label)
    if parent is not None:
        graph.edge(parent, curr_node)
    for child in node.children:
        create_treesitter_graph(child, source_code, graph, curr_node, counter, format)
    # Check if it's a leaf node and has source code
    if not node.children:
        node_source_code = source_code[node.start_byte:node.end_byte]
        if node_source_code and node_source_code != node.type:
            counter['i'] += 1
            source_code_node = 'node' + str(counter['i'])
            graph.node(source_code_node, label=f"{node_source_code}", shape='box', style='dashed')
            graph.edge(curr_node, source_code_node, style='dashed')
    return graph


def find_descendant_of_type(node, target_type):
    """递归查找特定类型的后代节点"""
    if node.type == target_type:
        return node
    for child in node.children:
        result = find_descendant_of_type(child, target_type)
        if result:
            return result
    return None


def process_file(file_path, global_vars, static_global_vars, relationship, treeGraph=None):
    folder_path = os.path.dirname(file_path)
    bname = os.path.basename(file_path)
    base_name_without_extension = os.path.splitext(bname)[0]
    graph_name = os.path.join(folder_path, f"tree_{base_name_without_extension}.png")
    global_var = static_var = extern_var = []
    global_var_map, static_var_map = {}, {}
    typedef_structs, direct_structs = [], []
    typedef_name_def_map, direct_structs_name_def_map = {}, {}
    with open(file_path, 'r') as f:
        code = f.read()
    tree = parser.parse(bytes(code, "utf8"))    
    if treeGraph == "Tree":
        print("可视化treesitter", bname)
        graph = create_treesitter_graph(tree.root_node, code)
        graph.format = 'svg'
        graph_name = os.path.join(folder_path, "tree_" + base_name_without_extension + ".dot")
        graph.render(graph_name, view=False)
    else:
        print("目前统计var_fn_relation，不进行可视化",bname)
        
    def first_traverse(node):    # 管两个struct
        # 抓direct-structs，必须得有field_declaration_list {}，防止前向声明
        if node.type == 'translation_unit': 
            child_types = {child.type for child in node.children}
            if 'declaration' in child_types:
                for child in node.children:
                    if child.type == "declaration":
                        graph_child_types = {grandchild.type for grandchild in child.children}
                        if 'init_declarator' in graph_child_types:
                            for grandchild in child.children:
                                if grandchild.type == "init_declarator":
                                    for gg in grandchild.children:
                                        if gg.type == "identifier":
                                            key = code[gg.start_byte:gg.end_byte]
                                            value = code[child.start_byte:child.end_byte]
                                            if 'storage_class_specifier' in graph_child_types:
                                                for specifier_child in child.children:
                                                    if specifier_child.type == "storage_class_specifier" and specifier_child.children[0].type == "static":
                                                        static_var_map[key+"-"+bname] = value
                                            else:
                                                global_var_map[key] = value

                                        
                            
            if 'struct_specifier' in child_types:
                for child in node.children:
                    if child.type == "struct_specifier":
                        # print("🐷",get_node_source(child, code))
                        # time.sleep(3)
                        musthave_field_declaration_list = [grandchild for grandchild in child.children if grandchild.type == 'field_declaration_list']
                        # otherwise is struct forward declaration
                        type_identifiers_node = [grandchild for grandchild in child.children if grandchild.type == 'type_identifier']
                        if type_identifiers_node and musthave_field_declaration_list:
                            duan_短struct名 = get_node_source(type_identifiers_node[0], code)
                            wanzheng_完整_struct = get_node_source(child, code)
                            # print("🐎",duan_短struct名)
                            # time.sleep(1)
                            direct_structs.append(duan_短struct名)
                            direct_structs_name_def_map[duan_短struct名] = wanzheng_完整_struct
                
        # 抓typedef-structs
        if node.type == 'type_definition':
            child_types = {child.type for child in node.children}
            if 'typedef' in child_types and 'struct_specifier' in child_types:
                # 找到类型为type_identifier的子节点
                type_identifiers_node = [child for child in node.children if child.type == 'type_identifier']
                # 由于type_identifier可能只有一个，所以我们只获取第一个节点的源代码
                if type_identifiers_node:
                    duan_短struct名 = get_node_source(type_identifiers_node[0], code)
                    wanzheng_完整_struct = get_node_source(node, code)
                    print("🦀️",duan_短struct名)
                    typedef_structs.append(duan_短struct名) 
                    typedef_name_def_map[duan_短struct名] = wanzheng_完整_struct
                    # time.sleep(2)
        for child in node.children:
            first_traverse(child)


                
    
    def extract_function_signature_from_definition(func_def_node):
        for child in func_def_node.children:
            if child.type in ['function_declarator', 'pointer_declarator']:
                sibling_texts = []
                for sibling in child.parent.children:
                    sibling_texts.append(get_node_source(sibling, code))
                    sibling_texts.append(" ")
                    if sibling == child:
                        sibling_texts.append(str(child.start_point[0]))
                        break
                return ''.join(sibling_texts).strip()
        return ""
    # time.sleep(2)
    
    
    def traverse(node): 
        global total_get_node_source_calls        
        condition1_global_variable = node.type == 'identifier' and get_node_source(node, code) in global_vars
        condition2_typedef_struct =  node.type == 'type_identifier' and node.parent.type != "type_definition" and get_node_source(node, code) in typedef_structs
        condition3_direct_struct =  node.type == 'type_identifier' and node.parent.parent.type != "translation_unit" and get_node_source(node, code) in direct_structs
        # condition4比较复杂
        variable_name = get_node_source(node, code)   # 提取变量名
        # 构造带有文件名的键
        key_with_filename = f"{variable_name}-{bname}"
        condition4_static_variable = node.type == 'identifier' and key_with_filename in static_global_vars

        if condition2_typedef_struct:
            print("condition2_typedef_struct🐟", get_node_source(node, code))
        if condition3_direct_struct:
            print("condition3_direct_struct🐷", get_node_source(node, code))
            # time.sleep(1)
        # 提取变量名
        variable_name = get_node_source(node, code)
        # 如果满足 condition4_static_variable，则为变量名添加文件名
        if condition4_static_variable:
            variable_name = f"{variable_name}-{bname}"

        if condition1_global_variable or condition2_typedef_struct or condition3_direct_struct or condition4_static_variable:
            # 第一种情况，基于函数定义
            temp_node = node
            while temp_node and temp_node.type != 'function_definition':
                temp_node = temp_node.parent

            if temp_node and temp_node.type == 'function_definition':
                func_name = extract_function_signature_from_definition(temp_node)
                full_fn_name = func_name + " in " + file_path
                if "getargs" in func_name:
                    print("func_name1",func_name)
                    time.sleep(1)
                    
                fn_get_node_source_calls[full_fn_name] = fn_get_node_source_calls.get(full_fn_name, 0) + 1
                if variable_name not in relationship:
                    relationship[variable_name] = {}
                relationship[variable_name].setdefault(full_fn_name, 0)
                relationship[variable_name][full_fn_name] += 1
            
            # 第二种情况，基于compound_statement
            temp_node = node
            while temp_node and not (temp_node.type == 'compound_statement' and temp_node.parent and temp_node.parent.type == 'translation_unit'):
                temp_node = temp_node.parent

            if temp_node and temp_node.type == 'compound_statement':
                sibling_nodes = [sibling for sibling in temp_node.parent.children if sibling.start_byte < temp_node.start_byte and sibling.type == 'declaration'] # int getpagesize(void) __attribute__((__const__)) ; error.i 
                print("1")
                for closest_sibling_declarator in reversed(sibling_nodes):  # 从最近的declarator开始，然后继续遍历其他declarator
                    func_declarator_child = find_descendant_of_type(closest_sibling_declarator, 'function_declarator')
                    # print("2")
                    # time.sleep(1)
                    if func_declarator_child:
                        # print("3")
                        # time.sleep(1)
                        func_name = extract_function_signature_from_definition(func_declarator_child.parent)
                        if "getargs" in func_name:         
                            print("func_name2",func_name)
                            # time.sleep(1)
                        full_fn_name = func_name + " in " + file_path
                        fn_get_node_source_calls[full_fn_name] = fn_get_node_source_calls.get(full_fn_name, 0) + 1
                        if variable_name not in relationship:
                            relationship[variable_name] = {}
                        relationship[variable_name].setdefault(full_fn_name, 0)
                        relationship[variable_name][full_fn_name] += 1



        for child in node.children:
            traverse(child)
        
    first_traverse(tree.root_node)    
    traverse(tree.root_node)
    
    gs_map = (global_var_map, static_var_map)
    return gs_map, typedef_name_def_map, direct_structs_name_def_map, fn_get_node_source_calls, total_get_node_source_calls

def generate_svg(relationship, global_vars, static_global_vars):
    dot = graphviz.Digraph(format='svg', graph_attr={
        'rankdir': 'LR', 
        'nodesep': '1',
        'ranksep': '5',
    })
    added_nodes = set()

    for var, funcs in relationship.items():
        color = 'lightblue2' if var in static_global_vars else 'khaki1'
        label = f"{var}\n({global_vars.get(var, static_global_vars.get(var, ''))})"
        dot.node(var, label=label, shape='box', color=color, style='filled')
        for full_fn_name, count in funcs.items():
            if full_fn_name not in added_nodes:
                dot.node(full_fn_name, label=full_fn_name, shape='ellipse')
                added_nodes.add(full_fn_name)
            dot.edge(full_fn_name, var, label=str(count))
    
    dot.render(filename="relationship", cleanup=True)
    
    
    
def create_combined_dependency_graph(variable_dependencies, profiling_relationship, static_global_vars, global_vars):
    dot = Digraph('CombinedDependencies', graph_attr={'rankdir': 'LR', 'size': '12,12'})

    # Define node style for global and static global variables
    def var_style(var_name):
        if var_name in static_global_vars:
            return {"color": "khaki1", "fontcolor": "black", "fontname": "bold"}
        else:
            return {"color": "lightblue2", "fontcolor": "black", "fontname": "bold"}

    # 1. 全局变量之间的依赖关系。
    for variable, depends_on in variable_dependencies.items():
        label = f"{variable}\n({global_vars.get(variable, static_global_vars.get(variable, ''))})"
        dot.node(variable, label=label, shape='box', style='filled', **var_style(variable))
        for depend in depends_on:
            label_dep = f"{depend}\n({global_vars.get(depend, static_global_vars.get(depend, ''))})"
            dot.node(depend, label=label_dep, shape='box', style='filled', **var_style(depend))
            dot.edge(variable, depend, color="blue")

    # 2. 函数与全局变量之间的依赖关系。
    for var, funcs in profiling_relationship.items():
        label = f"{var}\n({global_vars.get(var, static_global_vars.get(var, ''))})"
        dot.node(var, label=label, shape='box', style='filled', **var_style(var))
        for func, count in funcs.items():
            dot.node(func, shape="ellipse", color="black")
            dot.edge(func, var, label=str(count), color="red")

    return dot


