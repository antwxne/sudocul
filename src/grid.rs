use std::{error::Error, fmt::Display, usize};

const fn rounded_sqrt(n: usize) -> usize {
    let mut i = 1;
    while i < n / 2 {
        if i * i >= n {
            return i;
        }
        i += 1;
    }
    return 0;
}

#[derive(Debug)]
pub struct Grid<const SIZE: usize> {
    pub grid: [[usize; SIZE]; SIZE],
}

impl<const SIZE: usize> Grid<SIZE> {
    const BOX_SIZE: usize = rounded_sqrt(SIZE);
    const EMPTY_CASE: usize = 0;

    pub fn from_csv<T: std::io::Read>(
        csv_reader: &mut csv::Reader<T>,
    ) -> Result<Self, Box<dyn Error>> {
        let mut new_grid_array: [[usize; SIZE]; SIZE] = [[0; SIZE]; SIZE];
        for (i, result) in csv_reader.records().enumerate() {
            if i >= SIZE {
                panic!("Maximun grid size: {}", SIZE);
            }
            let record = result?;
            new_grid_array[i] = record
                .iter()
                .map(|elem| {
                    elem.parse::<usize>()
                        .expect("Expect number only in the grid")
                })
                .collect::<Vec<usize>>()
                .as_slice()
                .try_into()?;
        }

        Ok(Grid {
            grid: new_grid_array,
        })
    }

    pub fn solve(&mut self) -> bool {
        let mut row = 0;
        let mut col = 0;

        if !self.find_empty_case(&mut row, &mut col) {
            return true;
        }
        for number in 1..=SIZE {
            if self.is_number_safe(&number, &row, &col) {
                self.grid[row][col] = number;
                if self.solve() {
                    return true;
                }
                self.grid[row][col] = Self::EMPTY_CASE;
            }
        }
        false
    }

    fn is_number_safe(&self, value: &usize, row: &usize, col: &usize) -> bool {
        !self.is_in_row(value, row)
            && !self.is_in_col(value, col)
            && !self.is_in_box(value, row, col)
            && self.grid[*row][*col] == Self::EMPTY_CASE
    }

    fn find_empty_case(&self, row: &mut usize, col: &mut usize) -> bool {
        while *row < SIZE {
            while *col < SIZE {
                if self.grid[*row][*col] == Self::EMPTY_CASE {
                    return true;
                }
                *col += 1;
            }
            *row += 1;
            *col = 0;
        }
        false
    }

    fn is_in_row(&self, value: &usize, row: &usize) -> bool {
        for col in 0..SIZE {
            if self.grid[*row][col] == *value {
                return true;
            }
        }
        false
    }

    fn is_in_col(&self, value: &usize, col: &usize) -> bool {
        for row in 0..SIZE {
            if self.grid[row][*col] == *value {
                return true;
            }
        }
        false
    }

    fn is_in_box(&self, value: &usize, row: &usize, col: &usize) -> bool {
        let box_start_row = row - row % Self::BOX_SIZE;
        let box_start_col = col - col % Self::BOX_SIZE;

        for i_row in 0..3 {
            for i_col in 0..3 {
                if self.grid[i_row + box_start_row][i_col + box_start_col] == *value {
                    return true;
                }
            }
        }
        false
    }
}

impl<const SIZE: usize> Display for Grid<SIZE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const LINE: &'static str = "--------------------";

        writeln!(f, "{}", &LINE)?;
        for row in self.grid {
            writeln!(
                f,
                "|{}|",
                row.map(|elem| {
                    if elem == Self::EMPTY_CASE {
                        return " ".to_string();
                    }
                    elem.to_string()
                })
                .join("|")
                .as_str()
            )?;
            writeln!(f, "{}", &LINE)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Grid;
    const EXEMPLE_9_X9_GRID: [[usize; 9]; 9] = [
        [4, 5, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 2, 0, 7, 0, 6, 3, 0],
        [0, 0, 0, 0, 0, 0, 0, 2, 8],
        [0, 0, 0, 9, 5, 0, 0, 0, 0],
        [0, 8, 6, 0, 0, 0, 2, 0, 0],
        [0, 2, 0, 6, 0, 0, 7, 5, 0],
        [0, 0, 0, 0, 0, 0, 4, 7, 6],
        [0, 7, 0, 0, 4, 5, 0, 0, 0],
        [0, 0, 8, 0, 0, 9, 0, 0, 0],
    ];

    #[test]
    fn is_in_row_ok() {
        let grid = Grid {
            grid: EXEMPLE_9_X9_GRID.clone(),
        };
        let got = grid.is_in_row(&2, &1);
        const EXPECTED: bool = true;
        assert_eq!(got, EXPECTED);
    }
    #[test]
    fn is_in_row_ko() {
        let grid = Grid {
            grid: EXEMPLE_9_X9_GRID.clone(),
        };
        let got = grid.is_in_row(&9, &1);
        const EXPECTED: bool = false;
        assert_eq!(got, EXPECTED);
    }
    #[test]
    fn is_in_col_ok() {
        let grid = Grid {
            grid: EXEMPLE_9_X9_GRID.clone(),
        };
        let got = grid.is_in_col(&2, &1);
        const EXPECTED: bool = true;
        assert_eq!(got, EXPECTED);
    }
    #[test]
    fn is_in_col_ko() {
        let grid = Grid {
            grid: EXEMPLE_9_X9_GRID.clone(),
        };
        let got = grid.is_in_col(&9, &1);
        const EXPECTED: bool = false;
        assert_eq!(got, EXPECTED);
    }
    #[test]
    fn is_in_box_ok() {
        let grid = Grid {
            grid: EXEMPLE_9_X9_GRID.clone(),
        };
        let got = grid.is_in_box(&4, &8, &7);
        const EXPECTED: bool = true;
        assert_eq!(got, EXPECTED);
    }
    #[test]
    fn is_in_box_ko() {
        let grid = Grid {
            grid: EXEMPLE_9_X9_GRID.clone(),
        };
        let got = grid.is_in_box(&9, &8, &7);
        const EXPECTED: bool = false;
        assert_eq!(got, EXPECTED);
    }
    #[test]
    fn is_number_safe_ok() {
        let grid = Grid {
            grid: EXEMPLE_9_X9_GRID.clone(),
        };
        let got = grid.is_number_safe(&3, &4, &4);
        const EXPECTED: bool = true;
        assert_eq!(got, EXPECTED);
    }
    #[test]
    fn is_number_safe_ko_non_empty() {
        let grid = Grid {
            grid: EXEMPLE_9_X9_GRID.clone(),
        };
        let got = grid.is_number_safe(&3, &3, &4);
        const EXPECTED: bool = false;
        assert_eq!(got, EXPECTED);
    }
    #[test]
    fn is_number_safe_ko_on_row() {
        let grid = Grid {
            grid: EXEMPLE_9_X9_GRID.clone(),
        };
        let got = grid.is_number_safe(&8, &4, &4);
        const EXPECTED: bool = false;
        assert_eq!(got, EXPECTED);
    }
    #[test]
    fn is_number_safe_ko_on_col() {
        let grid = Grid {
            grid: EXEMPLE_9_X9_GRID.clone(),
        };
        let got = grid.is_number_safe(&7, &4, &4);
        const EXPECTED: bool = false;
        assert_eq!(got, EXPECTED);
    }
    #[test]
    fn is_number_safe_ko_on_box() {
        let grid = Grid {
            grid: EXEMPLE_9_X9_GRID.clone(),
        };
        let got = grid.is_number_safe(&9, &4, &4);
        const EXPECTED: bool = false;
        assert_eq!(got, EXPECTED);
    }
    #[test]
    fn solve() {
        let mut grid = Grid {
            grid: EXEMPLE_9_X9_GRID.clone(),
        };
        const EXPECTED: [[usize; 9]; 9] = [
            [4, 5, 3, 8, 2, 6, 1, 9, 7],
            [8, 9, 2, 5, 7, 1, 6, 3, 4],
            [1, 6, 7, 4, 9, 3, 5, 2, 8],
            [7, 1, 4, 9, 5, 2, 8, 6, 3],
            [5, 8, 6, 1, 3, 7, 2, 4, 9],
            [3, 2, 9, 6, 8, 4, 7, 5, 1],
            [9, 3, 5, 2, 1, 8, 4, 7, 6],
            [6, 7, 1, 3, 4, 5, 9, 8, 2],
            [2, 4, 8, 7, 6, 9, 3, 1, 5],
        ];
        grid.solve();
        assert_eq!(grid.grid, EXPECTED);
    }
}
