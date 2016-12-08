import re

pattern = r'(\[[a-z]*\])'

def main():
    data = load_input('input-1.txt'); count = 0
    for line in data:
        split = re.split(pattern, line)
        hn = [i for i in split if '[' in i]; sn = [i for i in split if '[' not in i]
        for seq in sn:
            valid = [i for i in range(0, len(seq) - 2) if \
                seq[i] != seq[i+1] and \
                seq[i] == seq[i+2] and \
                [s for s in hn if (seq[i+1] + seq[i] + seq[i+1]) in s]]
            if valid:
                count += 1
                break
    print(count)


def load_input(path):
    with open(path, 'r') as input_file:
        data = [line.rstrip('\n') for line in input_file]
        return data

if __name__ == "__main__":
    main()
