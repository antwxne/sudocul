use std::fmt::Display;

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

pub struct Grid<const COLS: usize, const ROWS: usize> {
    pub grid: [[usize; COLS]; ROWS],
}

impl<const COLS: usize, const ROWS: usize> Grid<COLS, ROWS> {
    const BOX_SIZE: usize = rounded_sqrt(COLS);
    const EMPTY_CASE: usize = 0;

    #[allow(dead_code)]
    pub fn new() -> Self {
        Grid {
            grid: [[0; COLS]; ROWS],
        }
    }

    pub fn solve(&mut self) -> bool {
        let mut row = 0;
        let mut col = 0;

        if !self.find_empty_case(&mut row, &mut col) {
            return true;
        }
        for number in 1..=COLS {
            if self.number_is_safe(&row, &col, &number) {
                self.grid[row][col] = number;
                if self.solve() {
                    return true;
                }
                self.grid[row][col] = Self::EMPTY_CASE;
            }
        }
        false
    }

    fn number_is_safe(&self, row: &usize, col: &usize, value: &usize) -> bool {
        !self.is_in_row(value, row)
            && !self.is_in_col(value, col)
            && !self.is_in_box(value, row, col)
            && self.grid[*row][*col] == Self::EMPTY_CASE
    }

    fn find_empty_case(&self, row: &mut usize, col: &mut usize) -> bool {
        while *row < ROWS {
            while *col < COLS {
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
        for col in 0..COLS {
            if self.grid[*row][col] == *value {
                return true;
            }
        }
        false
    }

    fn is_in_col(&self, value: &usize, col: &usize) -> bool {
        for row in 0..ROWS {
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

impl<const COLS: usize, const ROWS: usize> Display for Grid<COLS, ROWS> {
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
