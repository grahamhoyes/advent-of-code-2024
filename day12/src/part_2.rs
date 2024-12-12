use aoc::grid_2d::{Board, Coord, Dir};
use itertools::Itertools;
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
        let mut sides = 0;
        let mut queue: Vec<Coord> = vec![start];

        while let Some(position) = queue.pop() {
            if visited.contains(&position) {
                continue;
            }

            visited.insert(position);
            region.push(position);

            // Direction of the neighbour and whether it is of the same type
            let neighbours: [(Dir, bool); 4] = Dir::cardinal().map(|dir| {
                let coord = position + dir;
                (dir, board.get(&coord) == Some(this_type))
            });

            for (a, b) in neighbours.iter().circular_tuple_windows() {
                let (dir_a, same_type_a) = a;
                let (dir_b, same_type_b) = b;

                if !same_type_a && !same_type_b {
                    // Outer corners: a and b are both a different type from this_type
                    sides += 1
                } else if *same_type_a && *same_type_b {
                    // Inner corners: a and b are both the same type as this_type, and there's
                    // a different type between them on the diagonal
                    let diagonal = position + *dir_a + *dir_b;
                    if board.get(&diagonal) != Some(this_type) {
                        sides += 1
                    }
                }
            }

            queue.extend(
                neighbours
                    .into_iter()
                    .filter(|(_, same_type)| *same_type)
                    .map(|(dir, _)| position + dir),
            );
        }

        res += sides * region.len()
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

        assert_eq!(res, 1206);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, 849332);
    }
}
