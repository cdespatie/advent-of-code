import re

pattern = r'([a-z][^\d]*)(\d*?)(?:\[[a-z]*?\])'

def main():
    data = load_input('input-1.txt')
    for line in data:
        grps = re.search(pattern, line).groups()
        name, sec_id = grps[0].replace('-', ' ').strip(), int(grps[1])
        decoded = ''.join([chr(ord('a') + (ord(char) - ord('a') + (sec_id % 26)) % \
            (ord('z') - ord('a') + 1)) if char != ' ' else ' ' for char in name])
        if ('northpole' in decoded):
            print(decoded, sec_id)

def load_input(path):
    with open(path, 'r') as input_file:
        data = [line.rstrip('\n') for line in input_file]
        return data

if __name__ == '__main__':
    main()
