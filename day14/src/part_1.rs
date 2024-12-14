use aoc::grid_2d::Coord;
use itertools::Itertools;

pub fn solution(input: &str, board_size: (i32, i32)) -> usize {
    let end_positions: Vec<Coord> = input
        .lines()
        .map(|l| {
            let (position, velocity): (Coord, Coord) = l
                .split_whitespace()
                .map(|part| {
                    part[2..]
                        .split(",")
                        .map(|num| num.parse::<i32>().unwrap())
                        .collect_tuple::<(i32, i32)>()
                        .unwrap()
                        .into()
                })
                .collect_tuple()
                .unwrap();

            let end = position + (velocity * 100);

            // Wrap the end position so that it's back on the board
            end.wrap_to_size(board_size)
        })
        .collect();

    let midpoints = (board_size.0 / 2, board_size.1 / 2);
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
