import time

class Computer:
    def __init__(self, opcodes):
        self.opcodes = opcodes
        self.position = 0

    def print_program(self):
        print(self.opcodes)

    def compute(self):
        while True:
            raw_code = self.get_code()
            code = int(str(raw_code)[-2:])
            parameter_modes = [int(x) for x in str(raw_code)[:-2]][::-1]

            time.sleep(0.5)
            print(f'Pos: {self.position}, Code: {code}, Param modes: {parameter_modes}')

            if code == 99:
                break
            elif code == 1:
                self.add(parameter_modes)
            elif code == 2:
                self.multiply(parameter_modes)
            elif code == 3:
                self.user_input()
            elif code == 4:
                self.user_output()

        return self.opcodes

    def add(self, parameter_modes):
        param_1 = self.get_value(self.position + 1, self.get_parameter(0, parameter_modes))
        param_2 = self.get_value(self.position + 2, self.get_parameter(0, parameter_modes))
        target = self.get_value(self.position + 3, 1)

        self.opcodes[target] = param_1 + param_2
        self.position += 4

    def multiply(self, parameter_modes):
        param_1 = self.get_value(self.position + 1, self.get_parameter(0, parameter_modes))
        param_2 = self.get_value(self.position + 2, self.get_parameter(0, parameter_modes))
        target = self.get_value(self.position + 3, 1)

        self.opcodes[target] = param_1 * param_2
        self.position += 4
        
    def user_input(self):
        target = self.get_value(self.position + 1, 1)
        val = int(input('Input value: '))
        self.opcodes[target] = val

        self.position += 2

    def user_output(self):
        target = self.get_value(self.position + 1, 1)
        print('Output: ' + str(self.opcodes[target]))

        self.position += 2

    def get_code(self):
        return self.opcodes[self.position]

    def get_value(self, target, parameter_mode):
        if parameter_mode == 0:
            return self.opcodes[self.opcodes[target]]
        elif parameter_mode == 1:
            return self.opcodes[target]

    def get_parameter(self, index, modes):
        if len(modes) == 0 or len(modes) >= index:
            return 0
        else:
            return modes[index]


def part1():
    computer = Computer(data())
    # computer.print_program()

    return computer.compute()

def compute(opcodes):
    position = 0

    while True:
        code = opcodes[position]

        if code == 99: # Term program
            break
        elif code == 1: # Add
            opcodes[opcodes[position + 3]] = opcodes[opcodes[position + 1]] + opcodes[opcodes[position + 2]]
        elif code == 2: # Multiply
            opcodes[opcodes[position + 3]] = opcodes[opcodes[position + 1]] * opcodes[opcodes[position + 2]]

        position += 4

    return opcodes

def data():
    return [int(x) for x in open('input.txt', 'r').read().split(',')]

if __name__ == '__main__':
    print(part1())

