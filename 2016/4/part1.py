import re
from collections import Counter

pattern = r'([a-z][^\d]*)(\d*?)(?:\[)([a-z]*?)(?:\])'

def main():
    data = load_input('input-1.txt'); count = 0
    for line in data:
        grps = re.search(pattern, line).groups()
        sorted_names = [i[0] for i in sorted(Counter(grps[0].replace('-', '')).most_common(), key=lambda x: (-x[1], x[0]))][:5]
        count = count + int(grps[1]) if ''.join(sorted_names) == grps[2] else count
    print(count)

def load_input(path):
    with open(path, 'r') as input_file:
        data = [line.rstrip('\n') for line in input_file]
        return data

if __name__ == '__main__':
    main()
