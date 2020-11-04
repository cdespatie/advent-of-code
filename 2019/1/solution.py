from math import floor

def part1():
    return sum([calc(n) for n in data()])

def part2():
    result = 0

    for n in data():
        while n > 0:
            n = calc(n)
            if n > 0: result += n

    return result

def calc(n):
    return floor(n / 3) - 2

def data():
    return [int(n) for n in open('input.txt', 'r').read().splitlines()]

if __name__ == "__main__":
    print(part1())
    print(part2())
