use crate::part_1::{parse_input, run_astar, Cell, State};
use aoc::grid_2d::Coord;
use std::collections::HashSet;

pub fn solution(input: &str) -> u32 {
    let board = parse_input(input);

    let (_cost, parents) = run_astar(&board).expect("No solution found");

    // `parents` maps us from state -> states we got there from.
    // Traverse our way back from end to start to find all paths.
    let mut paths: Vec<Vec<State>> = Vec::new();

    let start = board.find(&Cell::Start)[0];
    let end = board.find(&Cell::End)[0];

    let mut partial_paths: Vec<Vec<State>> = parents
        .iter()
        .filter(|(state, _)| state.position == end)
        .map(|(state, _)| vec![state.clone()])
        .collect();

    let mut visited: HashSet<Coord> = HashSet::new();

    while let Some(mut path) = partial_paths.pop() {
        let state = path.last().unwrap();
        visited.insert(state.position);

        if state.position == start {
            // Found a complete path
            path.reverse();
            // Not being used in this solution, but it's here to
            // copy in the future
            paths.push(path);
            continue;
        }

        if let Some(parents_set) = parents.get(state) {
            for parent in parents_set.iter() {
                // For each parent, a new path diverges
                let mut new_path = path.clone();
                new_path.push(parent.clone());
                partial_paths.push(new_path);
            }
        }
    }

    visited.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../example.txt");
        let res = solution(input);

        assert_eq!(res, 45);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, 692);
    }
}
