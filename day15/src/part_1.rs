use aoc::grid_2d::{Board, Coord, Dir};

pub fn parse_directions(directions: &str) -> Vec<Dir> {
    directions
        .replace("\n", "")
        .chars()
        .map(|c| match c {
            '^' => Dir::North,
            '>' => Dir::East,
            'v' => Dir::South,
            '<' => Dir::West,
            _ => panic!("Unrecognized character {}", c),
        })
        .collect()
}

/// Simulate the robot's movement in `dir` direction.
///
/// Returns the new position of the robot, and updates the board in-place.
pub fn run_step(board: &mut Board<char>, robot: Coord, dir: Dir) -> Coord {
    // Scan forward from the robot's position
    let mut next = robot + dir;
    while let Some(c) = board.get(&next) {
        match c {
            '#' => {
                // Wall - can't move in this direction
                return robot;
            }
            'O' => {
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
    // Fill it with a box
    board.set(&next, 'O');

    // The robot advances by one, and gets replaced with an empty space
    board.set(&robot, '.');

    let robot = robot + dir;
    board.set(&robot, '@');

    robot
}

pub fn solution(input: &str) -> i32 {
    let (board, directions) = input.split_once("\n\n").unwrap();
    let mut board = Board::from_str(board);

    let directions: Vec<Dir> = parse_directions(directions);

    let mut robot = board.find(&'@')[0];

    for dir in directions {
        robot = run_step(&mut board, robot, dir);
    }

    // Sum up the coordinates of the boxes
    board
        .find(&'O')
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

        assert_eq!(res, 10092);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, 1538871);
    }
}
