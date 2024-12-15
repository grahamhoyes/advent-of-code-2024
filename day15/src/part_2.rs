use crate::part_1;
use aoc::grid_2d::{Board, Coord, Dir};
use std::collections::HashSet;

/// Simulate the robot's movement in a horizontal direction.
///
/// Returns the new position of the robot, and updates the board in-place.
pub fn run_step_horizontal(board: &mut Board<char>, robot: Coord, dir: Dir) -> Coord {
    assert!(matches!(dir, Dir::East | Dir::West));

    // Same algorithm used by part 1, but we have to move everything manually
    let mut next = robot + dir;
    while let Some(c) = board.get(&next) {
        match c {
            '#' => {
                // Wall - can't move in this direction
                return robot;
            }
            '[' | ']' | 'O' => {
                // A box - add it to the stack being pushed
                next = next + dir;
            }
            '.' => {
                // An empty cell - this gets filled up
                break;
            }
            _ => panic!("Unrecognized character {}", c),
        }
    }

    // If we got here, `next` is an empty space.
    // Need to work backwards, shifting everything over.
    let backwards = dir.rotate_180();
    let mut to_shift = next + backwards;

    while let Some(c) = board.get(&to_shift) {
        match c {
            '[' | ']' => {
                // A box to move over
                board.set(&next, c);
                next = to_shift;
                to_shift = to_shift + backwards;
            }
            '@' => {
                // The robot, move it then we're done
                board.set(&next, '@');
                board.set(&to_shift, '.');
                return next;
            }
            _ => {
                panic!("Unexpected character {}", c);
            }
        }
    }

    unreachable!("Should not get here")
}

/// Simulate the robot's movement in a vertical direction.
///
/// Returns the new position of the robot, and updates the board in-place.
fn run_step_vertical(board: &mut Board<char>, robot: Coord, dir: Dir) -> Coord {
    assert!(matches!(dir, Dir::North | Dir::South));

    // Need to scan forward, adding boxes that are fully or half-touching to the
    // push queue.
    // Done when we either hit a wall (blocking the whole move), or empty the queue
    // meaning there's empty space.
    let mut queue: Vec<Coord> = vec![robot];
    let mut to_move: HashSet<Coord> = HashSet::new();

    while let Some(coord) = queue.pop() {
        if to_move.contains(&coord) {
            continue;
        }

        // Border wall stops us from going off the board, so .unwrap()
        // is safe
        match board.get(&coord).unwrap() {
            '#' => {
                // Wall - can't move in this direction
                return robot;
            }
            '[' => {
                // Start of a box, need to enqueue the other half
                // and what's forward
                queue.push(coord + Dir::East);
                queue.push(coord + dir);
            }
            ']' => {
                // Same thing
                queue.push(coord + Dir::West);
                queue.push(coord + dir);
            }
            '.' => {
                // Empty space, let the queue drain
                continue;
            }
            '@' => {
                // The robot, on the initial move
                queue.push(coord + dir);
            }
            c => panic!("Unexpected character {}", c),
        }

        to_move.insert(coord);
    }

    // Do the move. Everything in `to_move` gets moved by `dir,` and
    // the original positions are cleared. To do this safely, we need
    // to_move to be sorted by:
    // - Top row first if moving up
    // - Bottom row first if moving down
    let mut to_move: Vec<Coord> = to_move.into_iter().collect();

    // This sorts so top rows (lower index) are first, good for moving North
    to_move.sort_by(|a, b| a.0.cmp(&b.0));

    if matches!(dir, Dir::South) {
        to_move.reverse();
    }

    for coord in to_move {
        // Move in the direction, replace with a .
        let element = board.get(&coord).unwrap();

        board.set(&(coord + dir), element);
        board.set(&coord, '.');
    }

    // Update the robot's direction
    robot + dir
}

pub fn solution(input: &str) -> i32 {
    let (board, directions) = input.split_once("\n\n").unwrap();

    // Construct a board that's twice as wide.
    let matrix: Vec<Vec<char>> = board
        .lines()
        .map(|line| {
            let mut row = Vec::new();

            for c in line.chars() {
                match c {
                    '@' => {
                        row.push('@');
                        row.push('.');
                    }
                    'O' => {
                        row.push('[');
                        row.push(']');
                    }
                    '.' | '#' => {
                        row.push(c);
                        row.push(c);
                    }
                    _ => panic!("Unrecognized character {}", c),
                }
            }

            row
        })
        .collect();

    let mut board = Board::new(matrix);

    let directions: Vec<Dir> = part_1::parse_directions(directions);

    let mut robot = board.find(&'@')[0];

    for dir in directions {
        match dir {
            Dir::East | Dir::West => {
                robot = run_step_horizontal(&mut board, robot, dir);
            }
            Dir::North | Dir::South => {
                robot = run_step_vertical(&mut board, robot, dir);
            }
            _ => panic!("Non-cardinal direction {:?}", dir),
        }
    }

    // Sum up the coordinates of the boxes
    board
        .find(&'[')
        .into_iter()
        .map(|c| {
            // 100 * the distance from the top edge + distance from the left edge
            100 * c.0 + c.1
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

        assert_eq!(res, 9021);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, 1543338);
    }
}
