use aoc::grid_2d::{Board, Coord};
use std::collections::HashSet;

pub fn solution(input: &str) -> usize {
    let board = Board::from_str(input);

    let mut visited: HashSet<Coord> = HashSet::new();
    let mut res = 0;

    for start in board.positions() {
        if visited.contains(&start) {
            continue;
        }

        let this_type = board.get_unchecked(&start);

        // Basic DFS search of neighbouring cells of the same type
        let mut region: Vec<Coord> = Vec::new();
        let mut perimeter = 0;
        let mut queue: Vec<Coord> = vec![start];

        while let Some(position) = queue.pop() {
            if visited.contains(&position) {
                continue;
            }

            visited.insert(position);
            region.push(position);

            let neighbours: Vec<Coord> = position
                .cardinal_neighbours()
                .into_iter()
                .filter(|n| board.get(n) == Some(this_type))
                .collect();

            perimeter += 4 - neighbours.len();
            queue.extend(neighbours);
        }

        res += perimeter * region.len()
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../example.txt");
        let res = solution(input);

        assert_eq!(res, 1930);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, 1437300);
    }
}
