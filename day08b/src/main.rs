use aoc::grid_2d::{Board, Coord};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn solution(input: &str) -> usize {
    let board = Board::from_str(input);
    let antenna_positions: HashMap<char, Vec<Coord>> = board.find_positions(|c| *c != '.');

    let mut antinode_positions: HashSet<Coord> = HashSet::new();

    for positions in antenna_positions.values() {
        for (a, b) in positions.iter().tuple_combinations() {
            let diff = (b - a).simplify();

            // Walk backwards from a (including a itself) to find antinodes in that direction
            let mut coord: Coord = *a;
            while board.get(&coord).is_some() {
                antinode_positions.insert(coord);
                coord = coord - diff;
            }

            // Now walk forwards to find antinodes in the other direction
            coord = *a + diff;
            while board.get(&coord).is_some() {
                antinode_positions.insert(coord);
                coord = coord + diff;
            }
        }
    }

    antinode_positions.len()
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

        assert_eq!(res, 34);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, 1174);
    }
}
