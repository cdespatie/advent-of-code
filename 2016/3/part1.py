def main():
    lines = [[int(y) for y in x.split()] for x in load_input('input-1.txt')]
    valid = [x for x in lines if validate(x[0], x[1], x[2])]

    print(len(valid))

def validate(a, b, c):
    return a + b > c and b + c > a and a + c > b

def load_input(path):
    with open(path, 'r') as input_file:
        data = [line.strip().rstrip('\n') for line in input_file]
        return data

if __name__ == '__main__':
    main()

