def part1():
    opcodes = data()

    opcodes[1] = 12
    opcodes[2] = 2

    return compute(opcodes)

def part2():
    opcodes = data()

    for i in range(100):
        for j in range(100):
            opcodes[1] = i
            opcodes[2] = j

            result = compute(opcodes.copy())
            
            if result[0] == 19690720:
                return (100 * opcodes[1]) + opcodes[2]

def compute(opcodes):
    position = 0

    while True:
        code = opcodes[position]

        if code == 99:
            break
        elif code == 1:
            opcodes[opcodes[position + 3]] = opcodes[opcodes[position + 1]] + opcodes[opcodes[position + 2]]
        elif code == 2:
            opcodes[opcodes[position + 3]] = opcodes[opcodes[position + 1]] * opcodes[opcodes[position + 2]]

        position += 4

    return opcodes

def data():
    return [int(n) for n in open('input.txt', 'r').read().split(',')]

if __name__ == '__main__':
    print(part1())
    print(part2())
