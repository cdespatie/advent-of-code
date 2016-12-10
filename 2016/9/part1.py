import re

pattern = r'(\d+)x(\d+)'

def main():
    data = list(load_input('input-1.txt')[0]); counter = 0
    while len(data) > 0:
        char = data.pop(0)
        if char == '(':
            code = find_code(data)
            digits = [int(i) for i in re.search(pattern, code).groups()]
            pop_n(data, digits[0])
            counter += digits[0] * digits[1]
        else:
            counter += 1
    print(counter)


def pop_n(data, n):
    for i in range(n):
        data.pop(0)

def find_code(data):
    char = data.pop(0)
    code = '(' + char
    while char != ')':
        char = data.pop(0);
        code += char
    return code

def load_input(path):
    with open(path, 'r') as input_file:
        data = [line.rstrip('\n') for line in input_file]
        return data

if __name__ == '__main__':
    main()
