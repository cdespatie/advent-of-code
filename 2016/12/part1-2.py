def main():
    data = load_input('input-1.txt'); reg = { 'a': 0, 'b': 0, 'c': 0, 'd': 0 }; i = 0
    while i < len(data):
        instr = data[i].split(' '); skip = False
        if instr[0] == 'cpy':
            reg[instr[2]] = int(instr[1]) if instr[1].isdigit() else reg[instr[1]]
        elif instr[0] == 'inc':
            reg[instr[1]] += 1
        elif instr[0] == 'dec':
            reg[instr[1]] -= 1
        elif instr[0] == 'jnz':
            jump_val = int(instr[1]) if instr[1].isdigit() else reg[instr[1]]
            if jump_val != 0:
                i += int(instr[2]); skip = True
        if not skip:
            i += 1

    print(reg)

def load_input(path):
    with open(path, 'r') as input_file:
        data = [line.rstrip('\n') for line in input_file]
        return data

if __name__ == '__main__':
    main()

