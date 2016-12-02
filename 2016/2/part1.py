def main():
    print('Hi!')


def load_input(path):
    with open(path, 'r') as input_file:
        data = input_file.read().replace('\n', '')
        return data

if __name__ == '__main__':
    main()
