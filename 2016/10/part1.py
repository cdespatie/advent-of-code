import re

p = r'[a-z ]*(\d*)[a-z ]*(\d*)[a-z ]*(\d*)'

def main():
    data = load_input('input-1.txt'); bots = []
    bins = [[]]
    values = [i for i in data if 'value' in i]
    gives = [i for i in data if 'gives' in i]

    net = BotNet()

    for value in values:
        digits = [str(i) for i in re.search(p, value).groups() if i != '']
        bot = net.find_or_create(str(digits[1]))
        bot.give_item(digits[0])

    for give in gives:
        digits = [str(i) for i in re.search(p, give).groups() if i != '']
        split = give.split(' ')
        bot = net.find_or_create(str(digits[0]))
        bot.instr = Instruction(digits[1] if split[5] != 'output' else 'b' + str(digits[1]),\
                digits[2] if split[10] != 'output' else 'b' + str(digits[2]))


    while max([len(bot.items) for bot in net.bot_list if 'b' not in bot.id]) > 1:
        for bot in net.bot_list:
            if len(bot.items) > 1 and 'b' not in bot.id:
                b_low = net.find_or_create(bot.instr.target_low)
                b_high = net.find_or_create(bot.instr.target_high)
                b_low.give_item(bot.take_smallest())
                b_high.give_item(bot.take_biggest())



def test_bot(bot):
    if 17 in bot.items and 61 in bot.items:
        return True
    else:
        return False

class BotNet:
    def __init__(self):
        self.bot_list = []

    def find_or_create(self, number):
        bot = next((x for x in self.bot_list if x.id == number), None)
        if bot is None:
            bot = Bot(number)
            self.bot_list.append(bot)
        return bot

class Instruction:
    def __init__(self, target_1, target_2):
        self.target_low = target_1
        self.target_high = target_2

    def __str__(self):
        return 'Low: ' + str(self.target_low) + ', High: ' + str(self.target_high)


class Bot:
    def __init__(self, number):
        self.id = str(number)
        self.items = []
        self.instr = None

    def give_item(self, item):
        self.items.append(item)

    def take_biggest(self):
        return sorted(self.items).pop()

    def take_smallest(self):
        return sorted(self.items).pop(0)

def load_input(path):
    with open(path, 'r') as input_file:
        data = [line.rstrip('\n') for line in input_file]
        return data

if __name__ == '__main__':
    main()
