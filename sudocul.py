#!/usr/bin/env python3

from loading import *

grid = load_grid()

def is_not_in_the_line(line, value):
    for i in range (0, 9):
        if grid[line][i] == value:
            return False
    return True


def is_not_in_the_column(column, value):
    for i in range (0, 9):
        if grid[i][column] == value:
            return False
    return True


def is_not_in_the_block(pos_y, pos_x, value):
    i = pos_y - (pos_y % 3)
    j = pos_x - (pos_x % 3)
    y = i
    x = j
    while y < i + 3:
        while x < j + 3:
            if grid[y][x] == value:
                return False
            x += 1
        y += 1
    return True


def solver(pos):
    if pos == 9 * 9:
        return True
    y = int(pos / 9)
    x = int(pos % 9)
    if grid[y][x] != 0:
        return solver(pos + 1)
    for new_value in range (1, 10):
        if is_not_in_the_column(x, new_value) == True and is_not_in_the_line(y, new_value) == True and is_not_in_the_block(y, x, new_value) == True:
            grid[y][x] = new_value
            if solver(pos + 1) == True:
                return True
    grid[y][x] = 0
    return False


def display(grid_arr):
    for y in range(len(grid_arr)):
        if y % 3 == 0:
            print("-------------------------")
        print("|", end=' ')
        for x in range(len(grid_arr[y])):
            print(grid_arr[y][x], end=' ')
            if x % 3 == 2:
                print("|", end=' ')
        print("\n", end='')
    print("-------------------------")


def main():
    print("avant:")
    display(grid);
    solver(0);
    print("\n\napres:")
    display(grid);


if __name__ == "__main__":
    main()
