import re

p = r'\((\d+)x(\d+)\)'

def main():
    data = load_input('input-1.txt')[0]
    count = calc(data)
    print('Final: ' + str(count))

def calc(string):
    match = re.search(p, string)
    if match:
        prefix_val = len(string[:match.start()])
        left = string[match.end():match.end()+int(match.group(1))]
        left_val = calc(left) * int(match.group(2))
        right = string[match.end()+int(match.group(1)):]
        right_val = calc(right)

        return prefix_val + left_val + right_val
    else:
        return len(string)

def load_input(path):
    with open(path, 'r') as input_file:
        data = [line.rstrip('\n') for line in input_file]
        return data

if __name__ == '__main__':
    main()
