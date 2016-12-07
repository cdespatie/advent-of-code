import re

pattern = r'(\[[a-z]*\])'

def main():
    data = load_input('input-1.txt'); ans = 0
    for line in data:
        split_line = re.split(pattern, line); valid = False
        for split in split_line:
            if '[' in split and test_for_abba(split):
                valid = False
                break
            elif test_for_abba(split):
                valid = True
        if valid:
            ans += 1

    print(ans)


def test_for_abba(string):
    for i in range(0, len(string) - 3):
        if string[i] + string[i+1] == string[i+3] + string[i+2] and string[i] != string[i+1]:
            return True
    return False

def load_input(path):
    with open(path, 'r') as input_file:
        data = [line.rstrip('\n') for line in input_file]
        return data

if __name__ == "__main__":
    main()
