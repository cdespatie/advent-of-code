import hashlib
import re

salt = 'ahsbgdzn'
pattern = r'(.)\1{2}'

def main():
    found = []

    i = 0
    while len(found) < 64:
        md5_val = hashlib.md5((salt + str(i)).encode('utf-8')).hexdigest()
        matches = re.search(pattern, md5_val)

        if matches:
            target_char = matches.group(1)
            for j in range(i + 1, i + 1000):
                next_val = hashlib.md5((salt + str(j)).encode('utf-8')).hexdigest()
                if target_char * 5 in next_val:
                    found.append((i, md5_val))
        i += 1

    print(found[-1])

if __name__ == '__main__':
    main()

