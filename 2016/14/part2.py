import hashlib
import re

salt = 'ahsbgdzn'
pattern = r'(.)\1{2}'
cache = {}

def main():
    found = []

    i = 0
    while len(found) < 64:
        md5_val = ultra_hash(salt + str(i))
        matches = re.search(pattern, md5_val)

        if matches:
            target_char = matches.group(1)
            for j in range(i + 1, i + 1000):
                next_val = ultra_hash(salt + str(j))
                if target_char * 5 in next_val:
                    found.append((i, md5_val))
        i += 1

    print(found[-1])

def ultra_hash(data):
    orig = data
    if orig in cache:
        return cache[orig]
    for i in range(2017):
        data = hashlib.md5(data.encode('utf-8')).hexdigest()
    cache[orig] = data
    return data

if __name__ == '__main__':
    main()

