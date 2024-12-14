use std::collections::HashSet;

use crate::part_1::parse_input;
use aoc::grid_2d::{Board, Coord};
use aoc::visualize::prompt;

fn step(robots: &mut Vec<(Coord, Coord)>, board_size: (i32, i32)) {
    for (position, velocity) in robots.iter_mut() {
        *position = (*position + *velocity).wrap_to_size(board_size);
    }
}

/// Find the size of the largest contiguous region of non-empty cells
fn get_largest_region_size(board: &Board<char>) -> usize {
    // Very similar to day 12
    let mut visited: HashSet<Coord> = HashSet::new();

    let mut res = 0;

    for start in board.positions() {
        if visited.contains(&start) {
            continue;
        }

        if board.get(&start) != Some('#') {
            continue;
        }

        let mut region_size = 0;
        let mut queue: Vec<Coord> = vec![start];

        while let Some(position) = queue.pop() {
            if visited.contains(&position) {
                continue;
            }

            if board.get(&position) != Some('#') {
                continue;
            }

            visited.insert(position);
            region_size += 1;

            let neighbours = position.cardinal_neighbours();
            queue.extend(neighbours);
        }

        res = res.max(region_size);
    }

    res
}

pub fn solution(input: &str, board_size: (i32, i32)) -> usize {
    let mut robots = parse_input(input);

    let mut board = Board::from_size(board_size, '.');

    let mut iterations = 0;
    let mut largest_region = 0;

    loop {
        iterations += 1;
        step(&mut robots, board_size);

        for (position, _) in robots.iter() {
            board.set(position, '#');
        }

        let this_largest_region = get_largest_region_size(&board);

        if this_largest_region > largest_region {
            board.print();
            largest_region = this_largest_region;

            let t = prompt(&format!(
                "Iteration {}. Largest region {}. Type q to quit.",
                iterations, largest_region
            ));

            if t.as_str() == "q" {
                break;
            }
        }

        // Reset the board
        for (position, _) in robots.iter() {
            board.set(position, '.');
        }
    }

    iterations
}

// No unit tests for this one because it's done by manual inspection.
// On my input, the Christmas tree occurs after 8179 iterations.
// That takes 7 outputs of the above largest region printout, and about
// 30s.
