def main():
    keypad = ((1,2,3),(4,5,6),(7,8,9))
    x = 1; y = 1; ans = []
    lines = load_input('input-1.txt')
    for line in lines:
        for instr in line:
            if instr == 'R':
                y = (y + 1) if y < 2 else 2
            elif instr == 'L':
                y = (y - 1) if y > 0 else 0
            elif instr == 'U':
                x = (x - 1) if x > 0 else 0
            elif instr == 'D':
                x = (x + 1) if x < 2 else 2
        ans.append(keypad[x][y])
    print(ans)

def load_input(path):
    with open(path, 'r') as input_file:
        data = [line.rstrip('\n') for line in input_file]
        return data

if __name__ == '__main__':
    main()
