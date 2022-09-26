fn main() {
    println!("Hello, world!");
}

pub fn grid_size(grid: &Vec<usize>) -> usize {
    let vec_len = grid.len() as f32;
    vec_len.sqrt() as usize
}

pub fn is_valid_solution(grid: &Vec<usize>) -> bool {
    let grid_size = grid_size(grid);
    let mut found = Vec::new();
    for idx in (0..grid.len()).step_by(grid_size) {
        if found.contains(&grid[idx]) {
            return false;
        }
        found.push(grid[idx]);
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::{grid_size, is_valid_solution};
    use std::vec;

    // At first, I wanted to write a solver for a 2x2 grid.
    // One that takes a grid and a vocabulary and a function
    // that validates the temporary solution.
    // But then I found that detecting the validity of a
    // solution is actually the harder part and that I should work on that.

    #[test]
    fn can_find_duplicates_2_by_2() {
        let grid = vec![0; 4];
        assert_eq!(false, is_valid_solution(&grid));

        let grid = vec![0, 1, 0, 1];
        assert_eq!(false, is_valid_solution(&grid));
    }

    #[test]
    fn can_detect_valid_solution() {
        let grid = vec![0, 1, 1, 0];
        assert_eq!(true, is_valid_solution(&grid));

        let grid = vec![0, 1, 2, 1, 2, 0, 2, 0, 1];
        assert_eq!(true, is_valid_solution(&grid));
    }

    #[test]
    fn grid_size_is_4() {
        let grid = vec![0, 1, 1, 0];
        assert_eq!(2, grid_size(&grid));
    }

    #[test]
    fn grid_size_is_9() {
        let grid = vec![0, 1, 2, 1, 2, 0, 2, 0, 1];
        assert_eq!(3, grid_size(&grid));
    }
}
