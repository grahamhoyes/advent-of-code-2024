use crate::part_1::{run_astar, Cell};
use aoc::grid_2d::{Board, Coord};
use itertools::Itertools;

pub fn solution(input: &str, board_size: (usize, usize), initial_drop: usize) -> String {
    let mut board = Board::from_size(board_size, Cell::Empty);

    let mut input_iter = input.trim().lines().map(|line| {
        let (x, y) = line
            .split(",")
            .map(|s| s.parse::<i32>().unwrap())
            .collect_tuple()
            .unwrap();

        // Our coordinate system tracks distance from the top
        Coord(y, x)
    });

    // Drop the first pieces
    input_iter.by_ref().take(initial_drop).for_each(|pos| {
        board.set(&pos, Cell::Occupied);
    });

    let start: Coord = (0, 0).into();
    let target: Coord = (board_size.0 - 1, board_size.1 - 1).into();

    // Drop pieces one at a time now. A binary search would be
    // faster, but on release mode this runs in under 2 seconds
    // anyway.
    for (i, pos) in input_iter.enumerate() {
        board.set(&pos, Cell::Occupied);

        println!("Dropping coordinate {}: {:?}", i + 1024, pos);
        if run_astar(&board, start, target).is_none() {
            return format!("{},{}", pos.1, pos.0);
        }
    }

    panic!("No solution found")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../example.txt");
        let res = solution(input, (7, 7), 12);

        assert_eq!(res, "6,1");
    }

    // Runs in a few seconds on release mode, but takes a while
    // on debug/test profiles
    // #[test]
    // fn test_input() {
    //     let input = include_str!("../input.txt");
    //     let res = solution(input, (71, 71), 1024);
    //
    //     assert_eq!(res, "56,29");
    // }
}
