def part1():
    start = 246515
    end = 739105
    count = 0

    for n in range(start, end):
        if adjacent_check(n) and increase_check(n): count += 1

    return count

def part2():
    start = 246515
    end = 739105
    count = 0

    for n in range(start, end):
        if unique_adjacent_check(n) and increase_check(n): count += 1

    return count

def adjacent_check(num):
    num_list = [int(n) for n in str(num)]
    for i, n in enumerate(num_list):
        if i > 0 and n == num_list[i - 1]:
            return True

    return False

def unique_adjacent_check(num):
    num_list = [int(n) for n in str(num)]

    for i, n in enumerate(num_list):
        # Find pairs, then look ahead or behind the pair for duplicates.
        if i < (len(num_list) - 1) and n == num_list[i + 1]:
            if (i < 1 or n != num_list[i - 1]) and (i > (len(num_list) - 3) or n != num_list[i + 2]):
                return True

    return False

def increase_check(num):
    num_list = [int(n) for n in str(num)]
    for i, n in enumerate(num_list):
        if i > 0 and n < num_list[i - 1]:
            return False

    return True

if __name__ == '__main__':
    print(part1())
    print(part2())
