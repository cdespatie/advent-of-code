import re
from itertools import product
from pprint import pprint

pattern = r'(\d+).*(\d+)'

def main():
    screen = [['-' for i in range(10)] for x in range(6)]
    data = load_input('input-1.txt')
    for instr in data:
        digits = re.search(pattern, instr).groups()

    for i, j in product(range(4), range(3)):
        screen[i][j] = 'x'
    pprint(screen)


def load_input(path):
    with open(path, 'r') as input_file:
        data = [line.rstrip('\n') for line in input_file]
        return data

if __name__ == '__main__':
    main()
