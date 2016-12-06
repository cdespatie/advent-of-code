from collections import Counter

def main():
    data = load_input('input-1.txt')
    print(''.join([Counter(col).most_common()[0][0] for col in list(zip(*data))]))

def load_input(path):
    with open(path, 'r') as input_file:
        data = [line.rstrip('\n') for line in input_file]
        return data

if __name__ == '__main__':
    main()
