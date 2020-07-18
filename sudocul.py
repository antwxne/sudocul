#!/usr/bin/env python3
##
## EPITECH PROJECT, 2020
## sudocul
## File description:
## script to resolve sudoku
##

grid = [
    [0, 7, 0, 0, 6, 0, 8, 0, 0],
    [0, 3, 0, 0, 0, 0, 0, 0, 6],
    [0, 0, 0, 0, 0, 0, 0, 6, 0],
    [0, 0, 0, 0, 0, 0, 0, 2, 7],
    [0, 2, 9, 0, 5, 0, 0, 1, 0],
    [0, 0, 4, 0, 3, 0, 7, 8, 0],
    [0, 0, 0, 0, 0, 6, 5, 0, 4],
    [3, 0, 0, 2, 0, 0, 0, 0, 0],
    [0, 0, 6, 0, 8, 3, 0, 0, 0],
    [0, 0, 2, 0, 0, 9, 0, 7, 0]
]

def is_not_in_the_line(grid_arr, line, value):
    for i in range (len(grid_arr[line])):
        if grid_arr[line][i] == value:
            return false
    return true

def is_not_in_the_column(grid_arr, column, value):
    for i in range (len(grid_arr)):
        if grid_arr[i][column] == value:
            return false
    return true

def is_not_in_the_block(grid_arr, value, pos_y, pos_x):
    pos_y = pos_y - (pos_y % 3)
    pos_x = pos_x - (pos_x % 3)
    for x in range (pos_x, pos_x + 3):
        for y in range (pos_y, pos_y + 3):
            if grid_arr[y][x] == value:
                return false
    return true

def solver(grid_arr, pos_y, pos_x):
    for new_value in range (1, 9):
        if is_not_in_the_column(grid_arr, x, new_value) == true && is_not_in_the_line(grid_arr, y, new_value) == true:
            grid_arr[pos_y][pos_x] = new_value
        else:
        
        
    
