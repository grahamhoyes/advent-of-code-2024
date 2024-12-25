use aoc::grid_2d::Board;

#[derive(Debug, Eq, PartialEq)]
enum PatternType {
    Key,
    Lock,
}

#[derive(Debug)]
struct KeyOrLock {
    height: usize,
    heights: Vec<usize>,
    // Using a tagged enum rather than an enum with fields
    // to keep the typing simple
    pattern_type: PatternType,
}

impl KeyOrLock {
    pub fn from_str(raw: &str) -> Self {
        let board = Board::from_str(raw);
        let (rows, cols) = board.size();

        let is_lock = board.matrix[0].iter().all(|x| x == &'#');

        let heights: Vec<usize> = (0..cols)
            .map(|col| {
                if is_lock {
                    (1..rows)
                        .find(|&row| board.matrix[row][col] == '.')
                        .map(|row| row - 1)
                        .unwrap()
                } else {
                    (0..rows - 1)
                        .rev()
                        .find(|&row| board.matrix[row][col] == '.')
                        .map(|row| rows - row - 2)
                        .unwrap()
                }
            })
            .collect();

        Self {
            height: rows,
            heights,
            pattern_type: if is_lock {
                PatternType::Lock
            } else {
                PatternType::Key
            },
        }
    }

    pub fn matches(&self, other: &KeyOrLock) -> bool {
        self.pattern_type != other.pattern_type
            && self.height == other.height
            && self
                .heights
                .iter()
                .zip(&other.heights)
                .all(|(h1, h2)| h1 + h2 <= self.height - 2)
    }
}

pub fn solution(input: &str) -> usize {
    let (locks, keys): (Vec<_>, Vec<_>) = input
        .trim()
        .split("\n\n")
        .map(KeyOrLock::from_str)
        .partition(|item| matches!(item.pattern_type, PatternType::Lock));

    locks
        .into_iter()
        .map(|lock| keys.iter().filter(|key| lock.matches(key)).count())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../example.txt");
        let res = solution(input);

        assert_eq!(res, 3);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, 2950);
    }
}
