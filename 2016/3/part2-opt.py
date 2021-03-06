def main():
    rows = [[int(y) for y in x.split()] for x in load_input('input-1.txt')]
    flattened = [item for sublist in list(zip(*rows)) for item in sublist]
    triangles = [flattened[n:n+3] for n in range(0, len(flattened), 3)]
    valid = [y for y in [sorted(x) for x in triangles] if y[0] + y[1] > y[2]]

    print(len(valid))

def load_input(path):
    with open(path, 'r') as input_file:
        data = [line.strip().rstrip('\n') for line in input_file]
        return data

if __name__ == '__main__':
    main()

