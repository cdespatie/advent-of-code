import itertools
import copy

f_1_1 = ['E', 'SG', 'SM', 'PG', 'PM']
f_1_2 = ['E', 'SG', 'SM', 'PG', 'PM', 'EM', 'EG', 'DM', 'DG']
f_2 = ['TG', 'RG', 'RM', 'CG', 'CM']
f_3 = ['TM']
f_4 = []

floors_1 = [f_1_1, f_2, f_3, f_4]
floors_2 = [f_1_2, f_2, f_3, f_4]

found_states = set()

def main():
    root = Node(floors_2)
    print(find_sln([root], 0))


def find_sln(nodes, depth):
    next_layer = []
    print(depth, len(nodes))
    for node in nodes:
        if node.is_valid() and node.is_sln():
            return (node, depth)
        else:
            for move in get_all_moves(node):
                new_node = Node(node.exec_move(move))
                if new_node.is_valid() and not collision(new_node.data):
                    next_layer.append(new_node)
                    found_states.add(str(new_node))

    return find_sln(next_layer, depth + 1)

def collision(state):
    simplified = str(state).replace('S', '').replace('P', '').replace('T', '').replace('R', '').replace('C', '') \
            .replace('E', '').replace('D', '')
    if simplified in found_states:
        return True
    else:
        found_states.add(simplified)
        return False

def get_all_moves(node):
    states = []; valid_moves = []
    elev_floor = node.find('E')

    items_on_floor = [i for i in node.data[elev_floor] if i != 'E']
    combinations = list(itertools.combinations(items_on_floor, 2)) + list(itertools.combinations(items_on_floor, 1))

    if elev_floor > 0 and elev_floor < 3:
        moves = [[i, 'up'] for i in combinations] + [[i, 'down'] for i in combinations]
    elif elev_floor == 3:
        moves = [[i, 'down'] for i in combinations]
    elif elev_floor == 0:
        moves = [[i, 'up'] for i in combinations]

    return moves

class Node:
    def __init__(self, data=[[]]):
        self.children = []
        self.data = data

    def __repr__(self):
        return str(self.data)

    def exec_move(self, move):
        cur_floor = self.find('E'); state = copy.deepcopy(self.data)
        for device in move[0]:
            state[cur_floor].remove(device)
            if move[1] == 'up':
                state[cur_floor+1].append(device)
            elif move[1] == 'down':
                state[cur_floor-1].append(device)
        if move[1] == 'up':
            state[cur_floor+1].append('E')
        elif move[1] == 'down':
            state[cur_floor-1].append('E')
        state[cur_floor].remove('E')
        return state

    def find(self, item):
        for i, row in enumerate(self.data):
            if item in row:
                return i
        return None

    def is_sln(self):
        if len(self.data[3]) == 11:
            return True
        else:
            return False

    def is_valid(self):
        for row in self.data:
            for item in [i for i in row if i != 'E']:
                if item[1] == 'M' and (item[0] + 'G') not in row and any('G' in i for i in row):
                    return False
        return True

if __name__ == '__main__':
    main()

