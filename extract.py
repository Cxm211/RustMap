import os
import sys
import re

def extract_functions(file_path):
    """
    Extracts function signatures from a .c or .h file.
    """
    with open(file_path, 'r') as file:
        content = file.read()

    # Regular expression to match function declarations
    function_pattern = r'\b(\w+)\s+(\w+)\s*\(([^)]*)\)\s*{'
    functions = re.findall(function_pattern, content)

    # Store as a list of function signatures
    function_signatures = []
    for return_type, func_name, params in functions:
        function_signatures.append({
            "return_type": return_type,
            "name": func_name,
            "params": params
        })
    return function_signatures

def extract_globals(file_path):
    """
    Extracts global variable definitions from a .c or .h file.
    """
    with open(file_path, 'r') as file:
        content = file.read()

    # Regular expression to match global variable declarations
    global_pattern = r'\b(\w+)\s+(\w+)\s*;'
    globals_found = re.findall(global_pattern, content)

    global_variables = []
    for var_type, var_name in globals_found:
        global_variables.append({
            "type": var_type,
            "name": var_name
        })
    return global_variables

def process_folder(folder_path):
    """
    Processes all .c and .h files in the specified folder to extract functions and globals.
    """
    scaffolding = {
        "functions": [],
        "global_variables": []
    }

    for root, _, files in os.walk(folder_path):
        for file in files:
            if file.endswith('.c') or file.endswith('.h'):
                file_path = os.path.join(root, file)
                functions = extract_functions(file_path)
                globals_vars = extract_globals(file_path)

                scaffolding["functions"].extend(functions)
                scaffolding["global_variables"].extend(globals_vars)

    return scaffolding

def write_scaffolding_to_file(scaffolding, output_path):
    """
    Writes extracted scaffolding information to a JSON file.
    """
    import json
    with open(output_path, 'w') as f:
        json.dump(scaffolding, f, indent=4)

    print(f"Scaffolding information saved to {output_path}")

if __name__ == '__main__':
    if len(sys.argv) < 2:
        print("Please provide a folder path as a command-line argument.")
        sys.exit(1)

    folder_path = sys.argv[1]
    scaffolding = process_folder(folder_path)

    # Save the scaffolding to a JSON file in the same folder
    output_path = os.path.join(folder_path, "scaffolding.json")
    write_scaffolding_to_file(scaffolding, output_path)
