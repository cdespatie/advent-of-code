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

def part2():
    count = 0
    data = input()
    expr = re.compile('(\d*)-(\d*) (\w): (\w*)')

    for line in data:
        split_line = expr.findall(line)[0]
        index_1 = int(split_line[0]) - 1
        index_2 = int(split_line[1]) - 1
        target_char = split_line[2]
        text = split_line[3]

        if (text[index_1] == target_char) ^ (text[index_2] == target_char):
            count += 1

    return count

def input():
    return [item.rstrip() for item in open('input.txt', 'r')]

if __name__ == '__main__':
    print(part1())
    print(part2())
