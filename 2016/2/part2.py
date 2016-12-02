def main():
    keypad = [[1],[2,3,4],[5,6,7,8,9],['A','B','C'],['D']]
    x = 2; y = 0; ans = []
    lines = load_input('input-1.txt')
    for line in lines:
        for instr in line:


        ans.append(keypad[x][y])
    print(ans)


def load_input(path):
    with open(path, 'r') as input_file:
        data = [line.rstrip('\n') for line in input_file]
        return data

if __name__ == '__main__':
    main()
