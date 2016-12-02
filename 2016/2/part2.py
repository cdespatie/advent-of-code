def main():
    keypad = [[0,0,1,0,0],[0,2,3,4,0],[5,6,7,8,9],[0,'A','B','C',0],[0,0,'D',0,0]]
    x = 2; y = 0; ans = []; dim = len(keypad) - 1
    lines = load_input('input-1.txt')
    for line in lines:
        for instr in line:
            temp_x = x; temp_y = y
            if instr == 'R':
                temp_y = (y + 1) if y < dim else dim
            elif instr == 'L':
                temp_y = (y - 1) if y > 0 else 0
            elif instr == 'U':
                temp_x = (x - 1) if x > 0 else 0
            elif instr == 'D':
                temp_x = (x + 1) if x < dim else dim
            if keypad[temp_x][temp_y] != 0:
                x = temp_x; y = temp_y
        ans.append(keypad[x][y])
    print(ans)

def load_input(path):
    with open(path, 'r') as input_file:
        data = [line.rstrip('\n') for line in input_file]
        return data

if __name__ == '__main__':
    main()
