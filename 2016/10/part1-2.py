import re

p = r'[a-z ]*(\d*)[a-z ]*(\d*)[a-z ]*(\d*)'

def main():
    bots = { }; trig = True; target = None
    data = load_input('input-1.txt')
    set_values = [x for x in data if 'value' in x]; transfers = [x for x in data if 'gives' in x]

    # Set initial values.
    for set_value in set_values:
        match = re.search(p, set_value)
        if match.group(2) in bots:
            bots[match.group(2)].items.append(int(match.group(1)))
        else:
            bots[match.group(2)] = Bot(int(match.group(1)))

    # Give bots instructions
    for transfer in transfers:
        match = re.search(p, transfer); split = transfer.split(' ')
        t_1 = match.group(2) if split[5] != 'output' else 'b' + match.group(2)
        t_2 = match.group(3) if split[10] != 'output' else 'b' + match.group(3)

        if match.group(1) not in bots:
            bots[match.group(1)] = Bot()
        if t_1 not in bots:
            bots[t_1] = Bot()
        if t_2 not in bots:
            bots[t_2] = Bot()
        bots[match.group(1)].instr = (t_1, t_2)

    # Execute instructions.
    while trig:
        trig = False
        for name, bot in bots.items():
            if 'b' not in name and len(bot.items) > 1:
                trig = True
                bots[bot.instr[0]].items.append(min(bot.items))
                bots[bot.instr[1]].items.append(max(bot.items))
                bot.items = []
            
            if target is None:
                target = scan(bots, 17, 61)
                if target is not None:
                    print(target)

    print(bots['b0'].items[0] * bots['b1'].items[0] * bots['b2'].items[0])


def scan(bots, t_1, t_2):
    for name, bot in bots.items():
        if t_1 in bot.items and t_2 in bot.items:
            return name
    return None

class Bot:
    def __init__(self, item=None):
        self.instr = ()
        self.items = []
        if item is not None:
            self.items.append(item)

    def __repr__(self):
        return 'Instr:' + str(self.instr) + ', Items: ' + str(self.items)


def load_input(path):
    with open(path, 'r') as input_file:
        data = [line.rstrip('\n') for line in input_file]
        return data

if __name__ == '__main__':
    main()
