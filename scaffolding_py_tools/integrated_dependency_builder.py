import os
import subprocess
import sys
import time
from graphviz import Digraph
import graphviz

from combined_dependency_analyzer import extract_global_variables_from_ctags, process_file
import re



def process_label(label):
    label = label.replace("static", "").strip()
    if " " in label:
        before_space, after_space = label.split(" ", 1)
        return before_space + " " + after_space.replace(" ", "")
    return label


def extract_signature(label):
    """Extracts function signature from a label, up to and including the closing parenthesis."""
    return label.split(')')[0] + ')'


def process_relationships(variable_relationship, new_function_labels):
    revised_var_to_fn, fn_to_vars = {}, {}
    for var, funcs in variable_relationship.items():
        if var not in revised_var_to_fn:
            revised_var_to_fn[var] = []
        for func in funcs:
            func_label = new_function_labels.get(func, func) 
            revised_var_to_fn[var].append(func_label)
            
            if func_label not in fn_to_vars:
                fn_to_vars[func_label] = []
            fn_to_vars[func_label].append(var)

    return revised_var_to_fn, fn_to_vars

def create_integrated_dependency_graph(function_relations, function_labels, variable_relationship):
    dot = Digraph(format='svg', graph_attr={'rankdir': 'LR', 'nodesep': '0.5', 'ranksep': '5'})

    # Update function labels to contain just the function signature
    new_function_labels = {}
    for func, label in function_labels.items():
        new_function_labels[func] = extract_signature(label)

    # Use function labels as keys in function_relations
    call_graph_relations = {}
    for caller, callees in function_relations.items():
        caller_label = new_function_labels.get(caller, caller)
        call_graph_relations[caller_label] = callees
        print("🔀 Replaced caller:", caller, "with label:", caller_label)
    # 创建一个集合来存储第一张图中的所有函数标签：
    function_labels_in_graph1 = set()

    # 在处理第二张图时，仅当 p_funcLabel 存在于上述集合中时，才添加它：
    for caller_label, callees in call_graph_relations.items():
        p_caller_label = process_label(caller_label)
        function_labels_in_graph1.add(p_caller_label)
        for callee in callees:
            callee_label = new_function_labels.get(callee, callee)  # Get the label of the callee
            p_callee_label = process_label(callee_label)
            print("⬆️caller label:", caller_label, ", callee label:", callee_label)
            dot.edge(p_caller_label, p_callee_label, color='black', style='solid')
            if "writeError" in caller_label:
                # Do something if writeError is a substring of caller_label
                print("'🔥writeError' is a substring of caller_label! caller_label:",caller_label)
                # time.sleep(5)
            else:
                # Do something else if it's not
                print("'writeError' is NOT a substring of caller_label.")


    weighted_edges = {}
    for var, funcs in variable_relationship.items():
        dot.node(var, shape='box', color='lightblue2', style='filled', fontcolor='black')
        print("🤔funcs", funcs)
        for func in funcs:
            func_label = new_function_labels.get(func, func)  # Get the label of the function
            total_usages = sum(funcs.values())
            func_label = extract_signature(func_label)
            func_label = func_label.replace("static", "")
            p_funcLabel = process_label(func_label)      
            if p_funcLabel not in function_labels_in_graph1: continue         
            print("2🌟",p_funcLabel)
            edge_key = (p_funcLabel, var)
            weighted_edges[edge_key] = total_usages
            dot.edge(p_funcLabel, var, label=str(total_usages), color='blue', style='dashed')
            
    revised_var_to_fn, fn_to_vars = process_relationships(variable_relationship, new_function_labels)
    return dot, revised_var_to_fn, fn_to_vars, weighted_edges

