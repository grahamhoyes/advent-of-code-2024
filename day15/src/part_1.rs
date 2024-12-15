use aoc::grid_2d::{Board, Dir};

pub fn solution(input: &str) -> i32 {
    let (board, directions) = input.split_once("\n\n").unwrap();
    let mut board = Board::from_str(board);

    let directions: Vec<Dir> = directions
        .replace("\n", "")
        .chars()
        .map(|c| match c {
            '^' => Dir::North,
            '>' => Dir::East,
            'v' => Dir::South,
            '<' => Dir::West,
            _ => panic!("Unrecognized character {}", c),
        })
        .collect();

    let mut robot = board.find(&'@')[0];

    'dirs: for dir in directions {
        // Scan forward from the robot's position
        let mut next = robot + dir;
        while let Some(c) = board.get(&next) {
            match c {
                '#' => {
                    // Wall - can't move in this direction
                    continue 'dirs;
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

        robot = robot + dir;
        board.set(&robot, '@');
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
