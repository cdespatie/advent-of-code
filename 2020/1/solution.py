def part1():
    numbers = data()

    for i, item1 in enumerate(numbers):
        for item2 in numbers[i:]:
            if item1 + item2 == 2020:
                    return item1 * item2

    return 'not found'

def part2():
    numbers = data()

    for i, item1 in enumerate(numbers):
        for j, item2 in enumerate(numbers[i:]):
            for item3 in numbers[j:]:
                if item1 + item2 + item3 == 2020:
                    return item1 * item2 * item3

    return 'not found'

def data():
    return [int(line.rstrip()) for line in open('input.txt', 'r')]

if __name__ == '__main__':
    print(part1())
    print(part2())
