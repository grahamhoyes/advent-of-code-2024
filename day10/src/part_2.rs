use aoc::grid_2d::{Board, Dir};

pub fn solution(input: &str) -> usize {
    let board = Board::transform_from_str(input, |c| c.to_digit(10).unwrap());

    let trailheads = board.find(&0);

    trailheads
        .into_iter()
        .map(|start| {
            let mut destinations = 0;
            let mut queue = vec![start];

            while let Some(current) = queue.pop() {
                let Some(current_height) = board.get(&current) else {
                    // Off the board
                    continue;
                };

                if current_height == 9 {
                    // End of the trail
                    destinations += 1;
                    continue;
                }

                for dir in Dir::cardinal() {
                    let next = current + dir;
                    let Some(next_height) = board.get(&next) else {
                        continue;
                    };

                    if next_height == current_height + 1 {
                        queue.push(next);
                    }
                }
            }

            destinations
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

        assert_eq!(res, 81);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, 1324);
    }
}
