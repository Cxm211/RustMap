
import graphviz

COLORS = ['red', 'green', 'blue', 'yellow', 'pink', 'cyan', 'magenta', 'orange', 'purple', 'brown']

def tarjan_scc(adj_list):
    index = 0
    stack = []
    lowlinks = {}
    index_map = {}
    sccs = []

    def strongconnect(node):
        nonlocal index
        index_map[node] = index
        lowlinks[node] = index
        index += 1
        stack.append(node)

        for neighbor in adj_list.get(node, []):
            if neighbor not in index_map:
                strongconnect(neighbor)
                lowlinks[node] = min(lowlinks[node], lowlinks[neighbor])
            elif neighbor in stack:
                lowlinks[node] = min(lowlinks[node], index_map[neighbor])

        if lowlinks[node] == index_map[node]:
            scc = []
            while True:
                w = stack.pop()
                scc.append(w)
                if w == node:
                    break
            sccs.append(scc)

    for node in adj_list:
        if node not in index_map:
            strongconnect(node)

    return sccs

def generate_graph(adj_list, sccs=None, highlighted=None):
    dot = graphviz.Digraph(format='svg', node_attr={'style': 'filled', 'shape': 'circle'})

    if sccs:
        color_map = {tuple(scc): COLORS[i % len(COLORS)] for i, scc in enumerate(sccs)}
        for scc, color in color_map.items():
            for node in scc:
                if highlighted and node in highlighted:
                    dot.node(node, color=color, shape='doublecircle')
                else:
                    dot.node(node, color=color)

        for node, neighbors in adj_list.items():
            for neighbor in neighbors:
                dot.edge(node, neighbor)
    else:
        for node, neighbors in adj_list.items():
            dot.node(node)
            for neighbor in neighbors:
                dot.edge(node, neighbor)

    return dot



def is_valid_scc(scc, all_sccs):
    scc_set = set(scc)
    for known_scc in all_sccs:
        if set(known_scc) == scc_set:
            return True
    return False

def get_user_input_scc(all_sccs):
    print("Available SCCs are:", all_sccs)
    while True:
        try:
            scc_input = input("Enter an SCC (comma separated nodes, e.g., 'E,D'): ").split(',')
            if is_valid_scc(scc_input, all_sccs):
                return scc_input
            else:
                print("The provided nodes do not form a valid SCC. Please try again.")
        except Exception as e:
            print("Error:", e)
            print("Please try again.")

def reachable_sccs_from(scc, adj_list, all_sccs):
    visited = set()
    queue = list(scc)
    
    reachable_nodes = set()

    while queue:
        node = queue.pop()
        if node not in visited:
            visited.add(node)
            for neighbor in adj_list.get(node, []):
                if neighbor not in visited:
                    queue.append(neighbor)
                    reachable_nodes.add(neighbor)

    reachable = []
    for other_scc in all_sccs:
        if other_scc != scc and any(node in reachable_nodes for node in other_scc):
            reachable.append(other_scc)

    return reachable

# Main execution
# ... [其他函数定义保持不变]

# Main execution
if __name__ == "__main__":
    adj_list = {
        'A': ['B'],
        'B': ['C'],
        'C': ['A'],
        'D': ['E', 'F'],
        'E': ['D', 'G'],
        'F': [],
        'G': ['H'],
        'H': ['G', 'I'],
        'I': []
    }

    sccs = tarjan_scc(adj_list)

    print("\nDetermining reachable SCCs from a user-defined SCC.\n")

    target_scc = get_user_input_scc(sccs)
    reachable = reachable_sccs_from(target_scc, adj_list, sccs)

    all_reachable_nodes = [node for scc in reachable for node in scc]
    highlighted_graph = generate_graph(adj_list, sccs, all_reachable_nodes)
    highlighted_graph.view(filename="highlighted_graph", directory="./", cleanup=True)
    
    print(f"\nFrom SCC {target_scc}, the reachable SCCs are:", reachable)

