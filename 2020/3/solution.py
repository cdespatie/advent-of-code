def part1():
    return solve(3, 1)

def part2():
    return solve(1, 1) * solve(3, 1) * solve(5, 1) * solve(7, 1) * solve(1, 2)

def solve(slope_x, slope_y):
    pos = 0
    count = 0
    hill = data()

    for row in hill[::slope_y]:
        if row[pos % len(row)] == '#':
            count += 1

        pos += slope_x

    return count

def data():
    return [line.rstrip() for line in open('input.txt', 'r')]

if __name__ == '__main__':
    print(part1())
    print(part2())
