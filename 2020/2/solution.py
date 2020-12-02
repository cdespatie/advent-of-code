import re

def part1():
    count = 0
    data = input()
    expr = re.compile('(\d*)-(\d*) (\w): (\w*)')

    for line in data:
        split_line = expr.findall(line)[0]
        min_freq = int(split_line[0])
        max_freq = int(split_line[1])
        target_char = split_line[2]
        text = split_line[3]

        freq = text.count(target_char)

        if freq <= max_freq and freq >= min_freq:
            count += 1

    return count

def input():
    return [item.rstrip() for item in open('input.txt', 'r')]

if __name__ == '__main__':
    print(part1())
