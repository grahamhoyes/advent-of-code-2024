use aoc::grid_2d::{Board, Coord};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn solution(input: &str) -> usize {
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
            let diff = b - a;
            let potential_antinodes = [
                b + &diff, // Extrapolate forward
                a - &diff, // Extrapolate backward
            ];

            potential_antinodes
                .into_iter()
                .filter(|coord| board.get(coord).is_some())
                .for_each(|coord| {
                    antinode_positions.insert(coord);
                });
        }
    }

    antinode_positions.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../example.txt");
        let res = solution(input);

        assert_eq!(res, 14);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, 332);
    }
}
