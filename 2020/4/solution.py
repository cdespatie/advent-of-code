def part1():
    valid_count = 0
    current_data = []
    passports = data()

    for line in passports:
        if len(line) == 0:
            if validate_passport(current_data):
                valid_count += 1

            current_data = []
            continue

        current_data.extend(line.split(' '))

    return valid_count

def part2():
    return 'hi!'

def validate_passport(passport):
    for item in ['byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid']:
        if all(item not in entries for entries in passport):
            return False

    return True

def super_validate_passport(passport):
    for item in ['byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid']:
        if all(item not in entries for entries in passport):
            return False

    return True

def data():
    return [line.strip('\n') for line in open('input.txt', 'r')]

if __name__ == '__main__':
    print(part1())
    print(part2())
