import sys
from enum import IntEnum

def main():

    input = load_file('input-1.txt')
    input_list = input.replace(',', '').split(' ')

    ans = go(input_list)

    blocks = calculate_blocks(ans)
    print(ans)
    print(blocks)


def go(input_list):
    facing = Direction.North
    coords = [0, 0]
    visited = [[0, 0]]

    for instr in input_list:
        direction = instr[0]
        dist = int(instr[1:])

        if direction == 'L':
            facing = change_direction(-1, facing)
        elif direction == 'R':
            facing = change_direction(1, facing)

        travelled = travel(coords, facing, dist)
        coords = change_coords(facing, dist, coords)
        
        for item in travelled:
            if item in visited:
                return item
            else:
                visited.append(item.copy())


def travel(start, direction, distance):
    travelled = []
    current = start.copy()

    for i in range(0, distance):
        travelled.append(change_coords(direction, 1, current).copy())

    return travelled


def calculate_blocks(coords):
    return abs(coords[0]) + abs(coords[1])


def change_coords(direction, distance, coords):
    if direction == Direction.North:
        coords[1] += distance
    elif direction == Direction.East:
        coords[0] += distance
    elif direction == Direction.South:
        coords[1] -= distance
    elif direction == Direction.West:
        coords[0] -= distance

    return coords


def change_direction(val, direction):
    new_dir = direction + val

    if new_dir > 3:
        new_dir = 0
    elif new_dir < 0:
        new_dir = 3

    return new_dir


def load_file(path):
    with open(path, 'r') as input_file:
        data = input_file.read().replace('\n', '')
        return data


class Direction(IntEnum):
    North = 0,
    East = 1,
    South = 2,
    West = 3


if __name__ == '__main__':
    main()

