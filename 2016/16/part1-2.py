state = '11110010111001001'
max_chars = 35651584

def main():
    new_data = fill_disk(state, max_chars)
    check = checksum(new_data)
    print(check)

def fill_disk(data, fill):
    while len(data) < fill:
        temp = ''.join(reversed(['1' if i == '0' else '0' for i in data]))
        data = data + '0' + temp
    return data[:fill]

def checksum(data):
    output = ''
    for i in range(0, len(data) - 1, 2):
        if data[i] == data[i+1]:
            output += '1'
        else:
            output += '0'
    if len(output) % 2 != 0:
        return output
    else:
        return checksum(output)

if __name__ == '__main__':
    main()
