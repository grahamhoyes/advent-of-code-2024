use aoc::grid_2d::{Board, Coord};
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum Cell {
    Empty,
    Occupied,
    Visited,
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Empty => write!(f, "."),
            Cell::Occupied => write!(f, "#"),
            Cell::Visited => write!(f, "X"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Visit {
    state: Coord,
    current_cost: u32,
    estimated_total_cost: u32,
}

impl Ord for Visit {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reversed ordering to make this a min-heap
        other.estimated_total_cost.cmp(&self.estimated_total_cost)
    }
}

impl PartialOrd for Visit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// A* again, just like Day 16 except no twist about rotation this time.
///
/// Returns the shortest number of steps.
pub fn run_astar(board: &Board<Cell>, start: Coord, target: Coord) -> Option<u32> {
    let mut costs: HashMap<Coord, u32> = HashMap::new();
    let mut visited: HashSet<Coord> = HashSet::new();
    let mut to_visit: BinaryHeap<Visit> = BinaryHeap::new();

    to_visit.push(Visit {
        state: start,
        current_cost: 0,
        estimated_total_cost: 0,
    });

    while let Some(Visit {
        state,
        current_cost,
        ..
    }) = to_visit.pop()
    {
        if matches!(board.get(&state), None | Some(Cell::Occupied)) {
            // Can't move here
            continue;
        }

        if !visited.insert(state) {
            // Already been here
            continue;
        }

        if state == target {
            return Some(current_cost);
        }

        for new_position in state.cardinal_neighbours() {
            let heuristic_cost = new_position.manhattan_distance(&target);
            let new_cost = current_cost + 1;

            let is_cheaper = costs
                .get(&new_position)
                .map_or(true, |&current| new_cost < current);

            if is_cheaper {
                costs.insert(new_position.clone(), new_cost);
                to_visit.push(Visit {
                    state: new_position,
                    current_cost: new_cost,
                    estimated_total_cost: new_cost + heuristic_cost,
                });
            }
        }
    }

    None
}

pub fn solution(input: &str, board_size: (usize, usize), initial_drop: usize) -> u32 {
    let mut board = Board::from_size(board_size, Cell::Empty);

    input.trim().lines().take(initial_drop).for_each(|line| {
        let (x, y) = line
            .split(",")
            .map(|s| s.parse::<i32>().unwrap())
            .collect_tuple()
            .unwrap();

        // Our coordinate system tracks distance from the top first
        let pos = Coord(y, x);

        board.set(&pos, Cell::Occupied);
    });

    let start: Coord = (0, 0).into();
    let target: Coord = (board_size.0 - 1, board_size.1 - 1).into();

    run_astar(&board, start, target).expect("No path found")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../example.txt");
        let res = solution(input, (7, 7), 12);

        assert_eq!(res, 22);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input, (71, 71), 1024);

        assert_eq!(res, 318);
    }
}
