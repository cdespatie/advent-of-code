import hashlib

door_id = 'ugkcyxxp'

def main():
    i = 0; pw = []
    while len(pw) < 8:
        hash_str = hashlib.md5((door_id + str(i)).encode('utf-8')).hexdigest()
        if hash_str[:5] == '00000':
            pw.append(hash_str[5])
            print(hash_str[5])
        i += 1
    print(pw)


if __name__ == '__main__':
    main()

