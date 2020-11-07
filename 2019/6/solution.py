class Node:
    def __init__(self, name):
        self.name = name
        self.children = []
        self.depth = None

    def add_child(self, child):
        self.children.append(child)

    def set_depth(self, depth):
        self.depth = depth

    def __eq__(self, another):
        return hasattr(another, 'name') and self.name == another.name

    def __hash__(self):
        return hash(self.name)

    def __repr__(self):
        return f'Name: {self.name}, Depth: {self.depth}, Children: {len(self.children)}'

def part1():
    graph = build_tree()
    depths(graph['COM'], 0)

    return sum([node.depth for (name, node) in graph.items()])

def part2():
    graph = build_tree
    # bfs(graph, graph['YOU'])


def build_tree():
    nodes = {}

    for node_pair in data():
        if node_pair[0] in nodes:
            left_node = nodes[node_pair[0]]
        else:
            left_node = Node(node_pair[0])
            nodes[left_node.name] = left_node

        if node_pair[1] in nodes:
            right_node = nodes[node_pair[1]]
        else:
            right_node = Node(node_pair[1])
            nodes[right_node.name] = right_node

        left_node.add_child(right_node)

    return nodes

def depths(node, depth):
    if node is not None:
        node.depth = depth

        for child in node.children:
            depths(child, depth + 1)

def lca(graph, node1, node2)

# def bfs(graph, node):
#     queue = [node]
#     discovered = {node}

#     while len(queue) > 0:
#         target = queue.pop()

#         if target.name == 'SAN':
#             return target

#         for child in target.children:
#             if child not in discovered:
#                 discovered.add(child)
#                 queue.append(child)


def data():
    return [x.strip().split(')') for x in open('input.txt', 'r').readlines()]

if __name__ == '__main__':
    # print(part1())
    print(part2())
