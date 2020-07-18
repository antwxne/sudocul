#!/usr/bin/env python3
##
## EPITECH PROJECT, 2020
## sudocul
## File description:
## load a grid
##

import sys
#from sudocul import grid

def get_nb(list):
    dest = []
    for i in range(1, len(list) - 1):
        dest.append(list[i].split(' '))
    return dest

def load_grid():
    buffer = open("grid.txt", "r")
    temp = buffer.read().split("\n")
    pos = 0
    if len(temp) != 14:
        print("mauvais nombre de lignes")
        sys.exit(84)
    tmp_grid = []
    for y in range(len(temp) - 1):
        if y % 4 != 0:
            tmp_grid.append([])
            for x in range(len(temp[y])):
                if temp[y][x].isdigit():
                    tmp_grid[pos].append(temp[y][x])
            pos += 1
    buffer.close()

if __name__ == "__main__":
    load_grid()