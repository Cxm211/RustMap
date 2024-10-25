import os
import subprocess
import sys
import graphviz


def generate_cflow_dot(folder_path):
    # Traverse all .i files in the folder
    c_files = [os.path.join(folder_path, f) for f in os.listdir(folder_path) if f.endswith('.i')]

    # Use all .i files as parameters for the cflow command
    cflow_command_input = ' '.join(c_files)

    all_dot_content = "digraph G {\n"
    all_dot_content += 'rankdir="LR";\n'  # Set layout from left to right

    # Call cflow to generate dot file
    command = f'cflow --format=dot {cflow_command_input}'
    process = subprocess.Popen(command, shell=True, stdout=subprocess.PIPE)
    dot_content = process.communicate()[0].decode()

    # Remove the header and footer from the generated content, as we only need the middle part
    dot_lines = dot_content.split('\n')[2:-2]

    for line in dot_lines:
        all_dot_content += line + "\n"

    all_dot_content += "}"

    return all_dot_content



if __name__ == '__main__':
    if len(sys.argv) < 2:
        print("Please provide a folder path as a command-line argument.")
        sys.exit(1)

    folder_path = sys.argv[1]
    dot_content = generate_cflow_dot(folder_path)

    # Use the graphviz library to render and generate an SVG file
    dot_graph = graphviz.Source(dot_content)

    # Modify the output path to place it in the user-provided folder
    callgraph_svg_path = os.path.join(folder_path, "scg")
    callgraph_dot_path = os.path.join(folder_path, "scg")

    dot_graph.render(filename=callgraph_svg_path, format="svg", view=False)
    dot_graph.render(filename=callgraph_dot_path, format="dot", view=False)

    print(f"Generated {callgraph_svg_path}")
