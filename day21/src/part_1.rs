use aoc::grid_2d::{Board, Coord, Dir};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Display, Formatter};
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
    Up,
    Down,
    Left,
    Right,
    Activate,
    Empty,
    Num(u8),
}

impl Display for Key {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Key::Up => write!(f, "^"),
            Key::Down => write!(f, "v"),
            Key::Left => write!(f, "<"),
            Key::Right => write!(f, ">"),
            Key::Activate => write!(f, "A"),
            Key::Empty => write!(f, " "),
            Key::Num(n) => write!(f, "{}", n),
        }
    }
}

impl From<char> for Key {
    fn from(value: char) -> Self {
        match value {
            num if num.is_numeric() => Key::Num(num.to_digit(10).unwrap() as u8),
            'A' => Key::Activate,
            '^' => Key::Up,
            '>' => Key::Right,
            'v' => Key::Down,
            '<' => Key::Left,
            _ => panic!("Invalid character: {}", value),
        }
    }
}

impl From<Dir> for Key {
    fn from(value: Dir) -> Self {
        match value {
            Dir::North => Key::Up,
            Dir::South => Key::Down,
            Dir::West => Key::Left,
            Dir::East => Key::Right,
            _ => panic!("Invalid direction to convert to Key: {:?}", value),
        }
    }
}

pub const NUMPAD: [[Key; 3]; 4] = [
    [Key::Num(7), Key::Num(8), Key::Num(9)],
    [Key::Num(4), Key::Num(5), Key::Num(6)],
    [Key::Num(1), Key::Num(2), Key::Num(3)],
    [Key::Empty, Key::Num(0), Key::Activate],
];

pub const DPAD: [[Key; 3]; 2] = [
    [Key::Empty, Key::Up, Key::Activate],
    [Key::Left, Key::Down, Key::Right],
];

pub struct Visit {
    position: Coord,
    path: Vec<Key>,
    visited: HashSet<Coord>,
}

/// Find all shortest paths between two keys
pub fn find_shortest_key_paths(board: &Board<Key>, source: Coord, target: Coord) -> Vec<Vec<Key>> {
    let mut paths: Vec<Vec<Key>> = Vec::new();

    // BFS to get the shortest path from source to target
    let mut to_visit: VecDeque<Visit> = VecDeque::from([Visit {
        position: source,
        path: Vec::new(),
        visited: HashSet::new(),
    }]);

    let mut shortest_path = usize::MAX;

    while let Some(Visit {
        position,
        mut path,
        mut visited,
    }) = to_visit.pop_front()
    {
        if position == target && path.len() <= shortest_path {
            shortest_path = path.len();

            // End with an activate, since we always want to press that key
            path.push(Key::Activate);

            // Normally we would need to clear paths if we found a longer
            // one previously, but with BFS / unweighted Dijkstra that isn't
            // going to happen.
            paths.push(path);

            continue;
        }

        if !visited.insert(position) {
            // Already been here
            continue;
        }

        let Some(elem) = board.get(&position) else {
            // Off the board
            continue;
        };

        if matches!(elem, Key::Empty) {
            // Can't move here
            continue;
        }

        for dir in Dir::cardinal() {
            let neighbour = position + dir;
            let mut path = path.clone();
            path.push(dir.into());
            to_visit.push_back(Visit {
                position: neighbour,
                path,
                visited: visited.clone(),
            })
        }
    }

    paths
}

pub type Paths = HashMap<(Key, Key), Vec<Vec<Key>>>;

/// Compute all shortest path direction sequences from
/// any position to any other, avoiding empty positions
pub fn find_all_shortest_paths(board: &Board<Key>) -> Paths {
    let mut paths: Paths = HashMap::new();

    for source in board.positions() {
        let source_element = board.get_unchecked(&source);
        if matches!(source_element, Key::Empty) {
            continue;
        }

        for target in board.positions() {
            let target_element = board.get_unchecked(&target);
            if matches!(target_element, Key::Empty) {
                continue;
            }

            paths.insert(
                (source_element, target_element),
                find_shortest_key_paths(board, source, target),
            );
        }
    }

    paths
}

/// Find the most optimal sequence of moves for a series of
/// robots controlling robots.
pub fn find_global_shortest_sequence(
    // The most optimal set of paths between two keys at a given level of
    // robotic depth. First element should be the numeric keypad for the
    // final robot, the rest should be dpads.
    paths: &[&Paths],
    // Sequence we're trying to optimize
    sequence: &[Key],
    // How many robots deep we are
    level: usize,
    // Memoization
    memo: &mut HashMap<(Vec<Key>, usize), usize>,
) -> usize {
    let cache_key = (sequence.to_vec(), level);
    if let Some(&cached) = memo.get(&cache_key) {
        return cached;
    }

    // Put an Activate at the beginning of the sequence since that's
    // always where we start
    let iter = std::iter::once(Key::Activate).chain(sequence.iter().cloned());

    let result: usize = iter
        .clone()
        .zip(iter.skip(1))
        .map(|(from, to)| {
            let possible_paths = &paths[level][&(from, to)];

            if level == paths.len() - 1 {
                // We've gone through everything, no need to try any more combinations
                possible_paths[0].len()
            } else {
                // Find the shortest path from taking any of the current possible paths
                possible_paths
                    .iter()
                    .map(|path| find_global_shortest_sequence(paths, path, level + 1, memo))
                    .min()
                    .unwrap()
            }
        })
        .sum();

    memo.insert(cache_key, result);
    result
}

/// Get the paths through the numberpad and dpad
///
/// [Paths] is a map from (key, key) to all the possible shortest
/// sequence of dpad presses to get from the first key to the second.
pub fn get_keypad_paths() -> (Paths, Paths) {
    let dpad_board = Board::new(DPAD.map(Vec::from).into_iter().collect());
    let dpad_paths = find_all_shortest_paths(&dpad_board);

    let numpad_board = Board::new(NUMPAD.map(Vec::from).into_iter().collect());
    let numpad_paths = find_all_shortest_paths(&numpad_board);

    (numpad_paths, dpad_paths)
}

pub fn solution(input: &str) -> usize {
    let (numpad_paths, dpad_paths) = get_keypad_paths();

    let mut cache = HashMap::new();

    input
        .lines()
        .map(|sequence| {
            let numpad_sequence: Vec<Key> = sequence.chars().map(|c| c.into()).collect();
            let numeric_value = sequence[0..3].parse::<usize>().unwrap();

            let len = find_global_shortest_sequence(
                &[&numpad_paths, &dpad_paths, &dpad_paths],
                &numpad_sequence,
                0,
                &mut cache,
            );

            len * numeric_value
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../example.txt");
        let res = solution(input);

        assert_eq!(res, 126384);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, 128962);
    }
}
