use aoc::grid_2d::{Board, Coord, Dir};
use std::collections::{HashMap, HashSet};

/// Explore the track. Returns a map from position -> how far away the
/// end is.
pub fn explore_track(board: &Board<char>) -> HashMap<Coord, usize> {
    let start = board.find(&'S')[0];
    let end = board.find(&'E')[0];

    // Doing this as a DFS just since it's easy, even though its unnecessary
    let mut visited: HashSet<Coord> = HashSet::new();
    let mut frontier: Vec<(Coord, usize)> = vec![(start, 0)];
    let mut distance_from_start: HashMap<Coord, usize> = HashMap::from([(start, 0usize)]);

    while let Some((pos, distance)) = frontier.pop() {
        if !visited.insert(pos) {
            // Stops us from going backwards without having to track direction
            continue;
        }

        distance_from_start.insert(pos, distance);

        if pos == end {
            break;
        }

        for neighbour in pos.cardinal_neighbours() {
            if matches!(board.get(&neighbour), Some('#') | None) {
                continue;
            }

            frontier.push((neighbour, distance + 1));
        }
    }

    // Convert distance from start to distance from end
    let track_length = *distance_from_start.get(&end).unwrap();

    distance_from_start
        .into_iter()
        .map(|(pos, distance)| (pos, track_length - distance))
        .collect()
}

/// Do some cheating.
///
/// `distances` is the distance from a coordinate to the end of the track
///
/// Returns a map from time saved -> how many ways you could cheat to save
/// that much time
pub fn cheat(distances: HashMap<Coord, usize>) -> HashMap<usize, usize> {
    let mut times_saved: HashMap<usize, usize> = HashMap::new();

    for (starting_pos, distance_to_end) in distances.iter() {
        for dir in Dir::cardinal() {
            // Cheat by jumping two spaces
            let cheat_to = starting_pos + dir + dir;

            // distances only contains spaces on the track, so this
            // makes sure we aren't stuck in a wall
            if let Some(new_distance) = distances.get(&cheat_to) {
                // Account for the time we spent cheating
                let new_distance = new_distance + 2;
                if new_distance < *distance_to_end {
                    // We saved time
                    let time_saved = distance_to_end - new_distance;
                    *times_saved.entry(time_saved).or_default() += 1;
                }
            }
        }
    }

    times_saved
}

pub fn solution(input: &str) -> usize {
    let board = Board::from_str(input);

    let distances = explore_track(&board);

    let times_saved = cheat(distances);

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

        assert_eq!(res, 1463);
    }
}
