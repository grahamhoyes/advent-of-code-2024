use aoc::grid_2d::{Board, Coord, Dir};
use std::collections::HashSet;

#[derive(Debug, Clone)]
enum Cell {
    Empty,
    Occupied,
}

fn solution(input: &str) -> usize {
    let mut position: Coord = Coord(0, 0);

    let board: Vec<Vec<Cell>> = input
        .lines()
        .enumerate()
        .map(|(row, l)| {
            l.chars()
                .enumerate()
                .map(|(col, c)| match c {
                    '.' => Cell::Empty,
                    '#' => Cell::Occupied,
                    '^' => {
                        position = Coord(row as i32, col as i32);
                        Cell::Empty
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let board: Board<Cell> = Board::new(board);

    let mut dir = Dir::North;
    let mut visited: HashSet<Coord> = HashSet::new();

    while board.get(&position).is_some() {
        visited.insert(position);

        // Look ahead. If there's an obstacle, spin. Repeat until no obstruction.
        while let Some(ahead) = board.get(&(position + dir)) {
            if matches!(ahead, Cell::Empty) {
                break;
            }
            dir = dir.rotate_right();
        }

        position = position + dir;
    }

    visited.len()
}

fn main() {
    let input = include_str!("../input.txt");
    let res = solution(input);

    println!("Result: {}", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../example.txt");
        let res = solution(input);

        assert_eq!(res, 41);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, 5067);
    }
}
