fn main() {
    println!("Hello, world!");
}

pub struct Row<'a> {
    grid: &'a Grid,
    start_idx: usize,
    current_idx: usize,
}

impl Iterator for Row<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_idx < self.start_idx + self.grid.size {
            let value = self.grid.data[self.current_idx];
            self.current_idx += 1;
            Some(value)
        } else {
            None
        }
    }
}

pub struct Grid {
    data: Vec<usize>,
    size: usize,
}

impl Grid {
    pub fn new(data: Vec<usize>) -> Self {
        let size = Grid::grid_size(data.len());
        Grid { data, size }
    }
    pub fn grid_size(vec_len: usize) -> usize {
        let vec_len = vec_len as f32;
        vec_len.sqrt() as usize
    }
    pub fn rows(&self) -> impl Iterator<Item = Row> {
        (0..self.data.len()).step_by(self.size).map(|idx| Row {
            grid: self,
            start_idx: idx,
            current_idx: idx,
        })
    }
}

pub fn is_valid_solution(grid: &Grid) -> bool {
    // check rows
    for row in grid.rows() {
        let mut found = Vec::new();
        for value in row {
            if value > 0 {
                if found.contains(&value) {
                    return false;
                }
                found.push(value);
            }
        }
    }

    // check columns
    for col_start in 0..grid.size {
        let mut found = Vec::new();
        for idx in (col_start..grid.data.len()).step_by(grid.size) {
            if grid.data[idx] > 0 {
                if found.contains(&grid.data[idx]) {
                    return false;
                }
                found.push(grid.data[idx]);
            }
        }
    }

    // check boxes

    true
}

#[cfg(test)]
mod tests {
    use crate::{is_valid_solution, Grid};
    use std::vec;

    // At first, I wanted to write a solver for a 2x2 grid.
    // One that takes a grid and a vocabulary and a function
    // that validates the temporary solution.
    // But then I found that detecting the validity of a
    // solution is actually the harder part and that I should work on that.

    #[test]
    fn can_detect_invalid_solution() {
        let grid = Grid::new(vec![0, 1, 0, 1]);
        assert_eq!(false, is_valid_solution(&grid));

        let grid = Grid::new(vec![0, 1, 0, 1]);
        assert_eq!(false, is_valid_solution(&grid));

        let mut grid = vec![0; 81];
        grid[0] = 1;
        grid[1] = 1;
        let grid = Grid::new(grid);
        assert_eq!(false, is_valid_solution(&grid));

        let mut grid = vec![0; 81];
        grid[0] = 1;
        grid[9] = 1;
        let grid = Grid::new(grid);
        assert_eq!(false, is_valid_solution(&grid));

        // let mut grid = vec![0; 81];
        // grid[0] = 1;
        // grid[10] = 1;
        // let grid = Grid::new(grid);
        // assert_eq!(false, is_valid_solution(&grid));
    }

    #[test]
    fn can_detect_valid_solution() {
        let grid = Grid::new(vec![0, 1, 1, 0]);
        assert_eq!(true, is_valid_solution(&grid));

        let grid = Grid::new(vec![0, 1, 2, 1, 2, 0, 2, 0, 1]);
        assert_eq!(true, is_valid_solution(&grid));

        let grid = Grid::new(vec![
            5, 8, 9, 1, 3, 4, 2, 6, 7, 3, 1, 2, 7, 8, 6, 4, 9, 5, 4, 6, 7, 2, 5, 9, 3, 1, 8, 1, 7,
            3, 6, 2, 8, 5, 4, 9, 6, 5, 4, 9, 7, 3, 8, 2, 1, 2, 9, 8, 4, 1, 5, 7, 3, 6, 7, 3, 1, 8,
            9, 2, 6, 5, 4, 9, 4, 5, 3, 6, 7, 1, 8, 2, 8, 2, 6, 5, 4, 1, 9, 7, 3,
        ]);
        assert_eq!(true, is_valid_solution(&grid));
    }

    #[test]
    fn grid_size_is_4() {
        let grid = Grid::new(vec![0, 1, 1, 0]);
        assert_eq!(2, grid.size);
    }

    #[test]
    fn grid_size_is_9() {
        let grid = Grid::new(vec![0, 1, 2, 1, 2, 0, 2, 0, 1]);
        assert_eq!(3, grid.size);
    }

    #[test]
    fn can_get_first_row() {
        let grid = Grid::new(vec![0, 1, 2, 1, 2, 0, 2, 0, 1]);
        let mut first_row = grid.rows().next().unwrap();
        assert_eq!(0, first_row.next().unwrap());
        assert_eq!(1, first_row.next().unwrap());
        assert_eq!(2, first_row.next().unwrap());
        assert_eq!(None, first_row.next());
    }

    #[test]
    fn can_get_second_row() {
        let grid = Grid::new(vec![0, 1, 2, 1, 2, 0, 2, 0, 1]);
        let mut second_row = grid.rows().nth(1).unwrap();
        assert_eq!(1, second_row.next().unwrap());
        assert_eq!(2, second_row.next().unwrap());
        assert_eq!(0, second_row.next().unwrap());
        assert_eq!(None, second_row.next());
    }
}
