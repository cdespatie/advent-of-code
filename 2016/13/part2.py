import math

salt = 1358
directions = [(1, 0), (0, 1), (0, -1), (-1, 0)]

def main():
    start = (1, 1); max_steps = 50
    root = Node(start[0], start[1], 0, None)
    sln = bfs(root)

    print(sln)
    print(len(sln))


def bfs(root):
    queue = [root]; sln = []

    while len(queue) > 0:
        current = queue.pop(0)
        for node in [x for x in current.get_neighbours() if x.dist <= 50]:
            if node not in queue:
                queue.append(node)
        if current not in sln:
            sln.append(current)

    return sln

class Node():
    def __init__(self, x, y, dist, prev):
        self.x = x
        self.y = y
        self.dist = dist
        self.prev = prev

    def __eq__(self, other):
        return self.x == other.x and self.y == other.y

    def __repr__(self):
        return str(self.x) + ', ' + str(self.y) + ' (' + str(self.dist) + ')'

    def is_valid(self):
        if is_wall(self.x, self.y): return False
        elif self.x < 0 or self.y < 0: return False
        # elif self.dist > 50: return False
        else: return True

    def get_neighbours(self):
        nodes = []
        for move in directions:
            nodes.append(Node(self.x + move[0], self.y + move[1], self.dist + 1, self))
        return [x for x in nodes if x.is_valid()]

def is_wall(x, y):
    temp = (x * x) + (3 * x) + (2 * x * y) + y + (y * y) + salt
    if bin(temp).count('1') % 2 == 0: return False
    else: return True

if __name__ == '__main__':
    main()

