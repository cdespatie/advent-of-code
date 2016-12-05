import hashlib
import random
import string

door_id = 'ugkcyxxp'

def main():
    i = 0; pw = [''] * 8
    while pw.count('') > 0:
        hash_str = hashlib.md5((door_id + str(i)).encode('utf-8')).hexdigest()
        pos = int(hash_str[5], 16)
        if hash_str[:5] == '00000' and int(hash_str[5], 16) < 8 and pw[pos] == '':
            pw[pos] = hash_str[6]
        i += 1
        if i % 15000 == 0: hacker_print(pw)
    hacker_print(pw)

def hacker_print(pw):
    for char in pw:
        if char == '':
            print(random.choice(string.digits + string.ascii_letters), end='')
        else:
            print(color.CYAN + char + color.END, end='')
    print('', end='\r')

class color:
   CYAN = '\033[96m'
   END = '\033[0m'

if __name__ == '__main__':
    main()
