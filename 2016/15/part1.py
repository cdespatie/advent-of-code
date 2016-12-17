disks = [[13, 10],
        [17, 15],
        [19, 17],
        [7, 1],
        [5, 0],
        [3, 1],
        [11, 0]]

def main():
    count = 0
    while True:
        count += 1
        rotate(disks)
        if time(disks):
            break

    print(disks, count)

def rotate(disks):
    for disk in disks:
        disk[1] = (disk[1] + 1) if disk[1] < (disk[0] - 1) else 0

def time(disks):
    for i, disk in enumerate(disks):
        temp = disk[0] - 1
        for _ in range(i):
            temp = (temp - 1) if temp > 0 else (disk[0] - 1)
        if disk[1] != temp:
            return False

    return True

if __name__ == '__main__':
    main()

