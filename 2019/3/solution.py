def part1():
    wire1 = data()[0]
    wire2 = data()[1]

    path1 = path(wire1)
    path2 = path(wire2)

    intersects = find_intersects(path1, path2)

    return min([distance(n, (0, 0)) for n in intersects])

def part2():
    wire1 = data()[0]
    wire2 = data()[1]

    path1 = path(wire1)
    path2 = path(wire2)

    return min(steps_in_intersects(path1, path2))

def path(coords):
    path = []
    position = (0, 0)

    for coord in coords:
        for n in range(int(coord[1:])):
            position = move(position, coord[0])
            path.append(position)

    return path

def move(position, direction):
    if direction == 'U':
        return (position[0], position[1] + 1)
    elif direction == 'R':
        return (position[0] + 1, position[1])
    elif direction == 'D':
        return (position[0], position[1] - 1)
    elif direction == 'L':
        return (position[0] - 1, position[1])

def find_intersects(path1, path2):
    set1 = set(path1)
    return [n for n in path2 if n in set1]

def steps_in_intersects(path1, path2):
    intersects = find_intersects(path1, path2)
    return [path1.index(n) + path2.index(n) + 2 for n in intersects]

def distance(position1, position2):
    return abs(position1[0] - position2[0]) + abs(position1[1] - position2[1])

def data():
    return [n.split(',') for n in open('input.txt', 'r').read().splitlines()]

if __name__ == '__main__':
    print(part1())
    print(part2())
