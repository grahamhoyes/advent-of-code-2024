use aoc::grid_2d::Coord;
use itertools::Itertools;

/// Parse the input into pairs of (start position, velocity) vectors
pub fn parse_input(input: &str) -> Vec<(Coord, Coord)> {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|part| {
                    part[2..]
                        .split(",")
                        .map(|num| num.parse::<i32>().unwrap())
                        .collect_tuple::<(i32, i32)>()
                        .unwrap()
                        .into()
                })
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

pub fn solution(input: &str, board_size: (i32, i32)) -> usize {
    let midpoints = (board_size.0 / 2, board_size.1 / 2);

    let end_positions = parse_input(input).into_iter().map(|(position, velocity)| {
        let end = position + (velocity * 100);

        // Wrap the end position so that it's back on the board
        end.wrap_to_size(board_size)
    });

    let mut quadrant_counts = [0usize; 4];

    for coord in end_positions {
        #[allow(clippy::comparison_chain)]
        if coord.0 < midpoints.0 {
            if coord.1 < midpoints.1 {
                quadrant_counts[0] += 1;
            } else if coord.1 > midpoints.1 {
                quadrant_counts[1] += 1;
            }
        } else if coord.0 > midpoints.0 {
            if coord.1 < midpoints.1 {
                quadrant_counts[2] += 1;
            } else if coord.1 > midpoints.1 {
                quadrant_counts[3] += 1;
            }
        }
    }

    quadrant_counts.iter().product::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../example.txt");
        let res = solution(input, (11, 7));

        assert_eq!(res, 12);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input, (101, 103));

        assert_eq!(res, 232253028);
    }
}
