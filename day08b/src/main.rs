use aoc::grid_2d::{Board, Coord};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn solution(input: &str) -> usize {
    let mut antenna_positions: HashMap<char, Vec<Coord>> = HashMap::new();

    let matrix: Vec<Vec<char>> = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| {
                    if c != '.' {
                        antenna_positions
                            .entry(c)
                            .or_default()
                            .push(Coord(row as i32, col as i32));
                    }

                    c
                })
                .collect()
        })
        .collect();

    // Won't actually be using this for much but easy bounds checking
    let board = Board::new(matrix);

    let mut antinode_positions: HashSet<Coord> = HashSet::new();

    for (_letter, positions) in antenna_positions.iter() {
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
