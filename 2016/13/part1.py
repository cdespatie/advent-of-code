import math

salt = 1358
directions = [(1, 0), (0, 1), (0, -1), (-1, 0)]

def main():
    start = (1, 1); dest = (31, 39)
    sln = a_star(start, dest)
    print(sln)

# Using sexy algorithms instead of easy solutions! Yes!
def a_star(start, dest):
    closed_set = []
    open_set = [Node(start[0], start[1], 0, dest, None)]

    while len(open_set) > 0:
        current = min(open_set)
        if (current.x, current.y) == dest:
            return shortest_path(current)

        open_set.remove(current)
        closed_set.append(current)

        for neighbour in current.get_neighbours():
            if neighbour in closed_set:
                continue
            if neighbour not in open_set:
                open_set.append(neighbour)

    return None

def shortest_path(node):
    path = []
    while node.came_from is not None:
        path.append(node)
        node = node.came_from
    return list(reversed(path))

class Node:
    def __init__(self, x, y, g_score, dest, came_from):
        self.x = x; self.y = y
        self.dest = dest
        self.g_score = g_score
        self.f_score = self.get_f_score()
        self.came_from = came_from
        self.cost = self.f_score + self.g_score

    def __lt__(self, other):
        return self.cost < other.cost

    def __eq__(self, other):
        return self.x == other.x and self.y == other.y

    def __repr__(self):
        return str(self.x) + ', ' + str(self.y) + ' (' + str(self.g_score) + ')'

    def is_valid(self):
        if is_wall(self.x, self.y): return False
        elif self.x < 0 or self.y < 0: return False
        else: return True

    def get_f_score(self):
        return line_dist((self.x, self.y), self.dest) 

    def get_neighbours(self):
        nodes = []
        for move in directions:
            nodes.append(Node(self.x + move[0], self.y + move[1], self.g_score + 1, self.dest, self))
        return [x for x in nodes if x.is_valid()]

def line_dist(start, end):
    return math.sqrt(math.pow(start[0] - end[0], 2) + math.pow(start[1] - end[1], 2))

def is_wall(x, y):
    temp = (x * x) + (3 * x) + (2 * x * y) + y + (y * y) + salt
    if bin(temp).count('1') % 2 == 0: return False
    else: return True

if __name__ == '__main__':
    main()

