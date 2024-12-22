use std::collections::HashMap;

// Consider only the diagonal directions this time
const DIRECTIONS: [(isize, isize); 4] = [(1, -1), (1, 1), (-1, -1), (-1, 1)];

struct Board {
    width: usize,
    height: usize,
    grid: Vec<Vec<char>>,
}

impl Board {
    pub fn from_str(input: &str) -> Self {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        Board {
            width: grid[0].len(),
            height: grid.len(),
            grid,
        }
    }

    pub fn get(&self, row: isize, col: isize) -> Option<char> {
        if row < 0 || row as usize >= self.width || col < 0 || col as usize >= self.height {
            return None;
        }

        Some(self.grid[row as usize][col as usize])
    }

    pub fn check_word(&self, start: &(usize, usize), dir: &(isize, isize), word: &str) -> bool {
        for offset in 0..word.len() {
            let Some(c) = self.get(
                start.0 as isize + dir.0 * offset as isize,
                start.1 as isize + dir.1 * offset as isize,
            ) else {
                return false;
            };

            if c != word.as_bytes()[offset] as char {
                return false;
            }
        }
        true
    }
}

/// Strategy:
///  - Find all the MAS instances that occur on diagonals
///  - For each one, store the position of its center A
///  - Count the number of times an A is shared
pub fn solution(input: &str) -> usize {
    const WORD: &str = "MAS";

    let board = Board::from_str(input);

    let mut center_positions: HashMap<(isize, isize), usize> = HashMap::new();

    for row in 0..board.height {
        for col in 0..board.width {
            // DIRECTIONS is now only the diagonals
            for dir in DIRECTIONS.iter() {
                if board.check_word(&(row, col), dir, WORD) {
                    let center = (row as isize + dir.0, col as isize + dir.1);
                    *center_positions.entry(center).or_insert(0) += 1;
                }
            }
        }
    }

    center_positions
        .into_iter()
        .filter(|(_pos, count)| *count == 2)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../example.txt");
        let res = solution(input);

        assert_eq!(res, 9);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, 1910);
    }
}
