pub const DIRECTIONS: [(isize, isize); 8] = [
    (0, 1),
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
];

pub struct Board {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Vec<char>>,
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

pub fn solution(input: &str) -> usize {
    const WORD: &str = "XMAS";

    let board = Board::from_str(input);

    let mut count: usize = 0;

    for row in 0..board.height {
        for col in 0..board.width {
            for dir in DIRECTIONS.iter() {
                // In each direction, search for XMAS.
                if board.check_word(&(row, col), dir, WORD) {
                    count += 1
                }
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../example.txt");
        let res = solution(input);

        assert_eq!(res, 18);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, 2560);
    }
}
