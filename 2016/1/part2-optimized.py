def main():
    visited = []
    vec = (0, 1)
    x = 0; y = 0
    input_data = load_file('input-1.txt').split(', ')
    for instr in input_data:
        vec = (vec[1], -vec[0]) if instr[0] == 'R' else (-vec[1], vec[0])
        for step in range(int(instr[1:])):
            x += vec[0]
            y += vec[1]
            if (x, y) in visited:
                print(abs(x) + abs(y)); return
            visited.append((x, y))


def load_file(path):
    with open(path, 'r') as input_file:
        data = input_file.read().replace('\n', '')
        return data

if __name__ == '__main__':
    main()
