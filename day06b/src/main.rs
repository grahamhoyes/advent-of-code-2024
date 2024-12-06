use aoc::grid_2d::{Board, Coord, Dir};
use std::collections::HashSet;
#[derive(Debug, Clone)]
enum Cell {
    Empty,
    Occupied,
}

#[derive(Debug)]
enum Outcome {
    Left,
    Loop,
}

/// Walk around the board until either leaving or encountering a loop
fn walk_around(
    board: &Board<Cell>,
    start: Coord,
    start_dir: Dir,
) -> (Outcome, HashSet<(Coord, Dir)>) {
    let mut dir: Dir = start_dir;
    let mut position: Coord = start;

    let mut visited: HashSet<(Coord, Dir)> = HashSet::new();

    while board.get(&position).is_some() {
        if visited.contains(&(position, dir)) {
            return (Outcome::Loop, visited);
        }

        visited.insert((position, dir));

        // Look ahead. If there's an obstacle, spin. Repeat until no obstruction.
        while let Some(ahead) = board.get(&(position + dir)) {
            if matches!(ahead, Cell::Empty) {
                break;
            }
            dir = dir.rotate_right();
        }

        position = position + dir;
    }

    (Outcome::Left, visited)
}

fn solution(input: &str) -> usize {
    let mut starting_position: Coord = Coord(0, 0);

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
                        starting_position = Coord(row as i32, col as i32);
                        Cell::Empty
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let mut board: Board<Cell> = Board::new(board);
    let mut num_loops = 0;

    // Optimization: Instead of trying every position, we only need to try introducing
    // an obstacle on the original path.
    let (_, visited_with_direction) = walk_around(&board, starting_position, Dir::North);
    let visited: HashSet<Coord> = visited_with_direction
        .into_iter()
        .map(|(coord, _dir)| coord)
        .collect();

    for pos in visited.iter() {
        if *pos == starting_position {
            continue;
        }

        if matches!(board.get(&pos).unwrap(), Cell::Empty) {
            board.set(&pos, Cell::Occupied);

            let (outcome, _) = walk_around(&board, starting_position, Dir::North);
            if matches!(outcome, Outcome::Loop) {
                num_loops += 1;
            }

            board.set(&pos, Cell::Empty);
        }
    }

    num_loops
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

        assert_eq!(res, 6);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, 1793);
    }
}
