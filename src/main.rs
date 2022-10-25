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

pub struct Col<'a> {
    grid: &'a Grid,
    current_idx: usize,
}

impl Iterator for Col<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_idx < self.grid.size * self.grid.size {
            let value = self.grid.data[self.current_idx];
            self.current_idx += self.grid.size;
            Some(value)
        } else {
            None
        }
    }
}

pub struct Box<'a> {
    grid: &'a Grid,
    start_idx: usize,
    current_idx: usize,
}

impl Iterator for Box<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_idx - self.start_idx < 2 * self.grid.size + 3 {
            let value = self.grid.data[self.current_idx];
            self.current_idx += 1;
            if (self.current_idx - self.start_idx) % 3 == 0 {
                self.current_idx += 6;
            }
            Some(value)
        } else {
            None
        }
    }
}

/// A Grid is represented as a one-dimensional Vec of usize.
/// A Grid is assumed to be a square with a side length of `size` fields.
pub struct Grid {
    data: Vec<usize>,
    size: usize,
}

impl Grid {
    pub fn new(data: Vec<usize>) -> Self {
        let size = 9;
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

    pub fn cols(&self) -> impl Iterator<Item = Col> {
        (0..self.size).map(|idx| Col {
            grid: self,
            current_idx: idx,
        })
    }

    pub fn boxes(&self) -> impl Iterator<Item = Box> {
        [0, 3, 6, 27, 30, 33, 54, 57, 60]
            .into_iter()
            .map(|idx| Box {
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
    for col in grid.cols() {
        let mut found = Vec::new();
        for value in col {
            if value > 0 {
                if found.contains(&value) {
                    return false;
                }
                found.push(value);
            }
        }
    }

    // check boxes
    for bx in grid.boxes() {
        let mut found = Vec::new();
        for value in bx {
            if value > 0 {
                if found.contains(&value) {
                    return false;
                }
                found.push(value);
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::{is_valid_solution, Grid};
    use std::vec;

    #[test]
    fn can_detect_invalid_solution_in_9x9_row() {
        let mut grid = vec![0; 81];
        grid[0] = 1;
        grid[1] = 1;
        let grid = Grid::new(grid);
        assert_eq!(false, is_valid_solution(&grid));
    }

    #[test]
    fn can_detect_invalid_solution_in_9x9_col() {
        let mut grid = vec![0; 81];
        grid[0] = 1;
        grid[9] = 1;
        let grid = Grid::new(grid);
        assert_eq!(false, is_valid_solution(&grid));
    }

    #[test]
    fn can_detect_invalid_solution_in_9x9_box() {
        let mut grid = vec![0; 81];
        grid[0] = 1;
        grid[10] = 1;
        let grid = Grid::new(grid);
        assert_eq!(false, is_valid_solution(&grid));
    }

    #[test]
    fn can_detect_valid_solution() {
        let grid = Grid::new(vec![
            5, 8, 9, 1, 3, 4, 2, 6, 7,
            3, 1, 2, 7, 8, 6, 4, 9, 5,
            4, 6, 7, 2, 5, 9, 3, 1, 8,
            1, 7, 3, 6, 2, 8, 5, 4, 9,
            6, 5, 4, 9, 7, 3, 8, 2, 1,
            2, 9, 8, 4, 1, 5, 7, 3, 6,
            7, 3, 1, 8, 9, 2, 6, 5, 4,
            9, 4, 5, 3, 6, 7, 1, 8, 2,
            8, 2, 6, 5, 4, 1, 9, 7, 3,
        ]);
        assert_eq!(true, is_valid_solution(&grid));
    }

    #[test]
    fn can_get_first_row() {
        let grid = Grid::new(vec![
            5, 8, 9, 1, 3, 4, 2, 6, 7, 3, 1, 2, 7, 8, 6, 4, 9, 5, 4, 6, 7, 2, 5, 9, 3, 1, 8, 1, 7,
            3, 6, 2, 8, 5, 4, 9, 6, 5, 4, 9, 7, 3, 8, 2, 1, 2, 9, 8, 4, 1, 5, 7, 3, 6, 7, 3, 1, 8,
            9, 2, 6, 5, 4, 9, 4, 5, 3, 6, 7, 1, 8, 2, 8, 2, 6, 5, 4, 1, 9, 7, 3,
        ]);
        let mut first_row = grid.rows().next().unwrap();
        assert_eq!(5, first_row.next().unwrap());
        assert_eq!(8, first_row.next().unwrap());
        assert_eq!(9, first_row.next().unwrap());
    }

    #[test]
    fn can_get_second_row() {
        let grid = Grid::new(vec![
            5, 8, 9, 1, 3, 4, 2, 6, 7, 3, 1, 2, 7, 8, 6, 4, 9, 5, 4, 6, 7, 2, 5, 9, 3, 1, 8, 1, 7,
            3, 6, 2, 8, 5, 4, 9, 6, 5, 4, 9, 7, 3, 8, 2, 1, 2, 9, 8, 4, 1, 5, 7, 3, 6, 7, 3, 1, 8,
            9, 2, 6, 5, 4, 9, 4, 5, 3, 6, 7, 1, 8, 2, 8, 2, 6, 5, 4, 1, 9, 7, 3,
        ]);
        let mut second_row = grid.rows().nth(1).unwrap();
        assert_eq!(3, second_row.next().unwrap());
        assert_eq!(1, second_row.next().unwrap());
        assert_eq!(2, second_row.next().unwrap());
    }
}
