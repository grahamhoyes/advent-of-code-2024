use crate::part_1::explore_track;
use aoc::grid_2d::{Board, Coord};
use std::collections::HashMap;

/// Do some cheating, even more this time. Now we can cheat
/// for up to 20 turns.
///
/// `distances` is the distance from a coordinate to the end of the track
///
/// Returns a map from time saved -> how many ways you could cheat to save
/// that much time
pub fn cheat_harder(distances: HashMap<Coord, usize>) -> HashMap<usize, usize> {
    let mut times_saved: HashMap<usize, usize> = HashMap::new();

    for (starting_pos, distance_to_end) in distances.iter() {
        let Coord(start_row, start_col) = starting_pos;

        // Iterate over a box that's 41 long on each size, and filter
        // those down by manhattan distance
        for row in (start_row - 20)..=(start_row + 20) {
            for col in (start_col - 20)..=(start_col + 20) {
                let new_pos = Coord(row, col);
                let distance_cheated = new_pos.manhattan_distance(starting_pos) as usize;

                if distance_cheated > 20 {
                    continue;
                }

                if let Some(new_distance) = distances.get(&new_pos) {
                    let new_distance = new_distance + distance_cheated;

                    if new_distance < *distance_to_end {
                        // We saved time
                        let time_saved = distance_to_end - new_distance;
                        *times_saved.entry(time_saved).or_default() += 1;
                    }
                }
            }
        }
    }

    times_saved
}

pub fn solution(input: &str) -> usize {
    let board = Board::from_str(input);

    let distances = explore_track(&board);

    let times_saved = cheat_harder(distances);

    times_saved
        .into_iter()
        .filter(|(time_saved, _)| *time_saved >= 100)
        .map(|(_time, ways)| ways)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../example.txt");
        let res = solution(input);

        assert_eq!(res, 0);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, 985332);
    }
}
