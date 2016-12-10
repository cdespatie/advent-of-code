import re
from itertools import product

pattern = r'(\d+)[a-z ]*(\d+)'

def main():
    screen = [['-' for i in range(50)] for x in range(6)]
    data = load_input('input-1.txt')
    for instr in data:
        digits = [int(x) for x in re.search(pattern, instr).groups()]
        if 'rect' in instr: rect(screen, digits[0], digits[1])
        elif 'column' in instr: rotate_col(screen, digits[0], digits[1])
        elif 'row' in instr: rotate_row(screen, digits[0], digits[1])

    print(sum(i.count('x') for i in screen))
    pretty_print(screen)

def rotate_col(screen, x, y):
    for i in range(y):
        last = screen[-1][x]
        for j in reversed(range(len(screen))):
            screen[j][x] = screen[j-1][x]
        screen[0][x] = last

def rotate_row(screen, x, y):
    for i in range(y):
        screen[x].insert(0, screen[x].pop())

def rect(screen, x, y):
    for i, j in product(range(y), range(x)):
        screen[i][j] = 'x'

def pretty_print(matrix):
    for row in matrix:
        for item in row:
            print(item, end='')
        print()

def load_input(path):
    with open(path, 'r') as input_file:
        data = [line.rstrip('\n') for line in input_file]
        return data

if __name__ == '__main__':
    main()
