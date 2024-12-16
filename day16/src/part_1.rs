use aoc::grid_2d::{Board, Coord, Dir};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cell {
    Start,
    Empty,
    Wall,
    End,
}

/// Heuristic function for A*, which approximates the cost function for this problem:
/// Manhattan distance + 1000 * number of required rotations to point sort of
/// towards the target
pub fn heuristic(current: (Coord, Dir), target: Coord) -> u32 {
    let (current_pos, current_dir) = current;

    let direction_vec = target - current_pos;

    // Whichever coordinate has a greater magnitude can indicate roughly which
    // cardinal direction we should try to move in.
    // Since the end is always in the top right, the row coordinate will always
    // be <= 0 and the column >=0
    let (d_rows, d_cols) = (direction_vec.0.abs(), direction_vec.1.abs());
    let target_dir = match d_rows.cmp(&d_cols) {
        Ordering::Greater => Dir::North, // More rows to move than columns
        Ordering::Less => Dir::East,     // More columns to move than rows
        Ordering::Equal => current_dir,  // Doesn't matter, stay current
    };

    let rotations_required = (current_dir.offset_from(&target_dir) / 90).unsigned_abs();

    direction_vec.l1_norm() + 1000 * rotations_required
}

#[derive(Debug, PartialEq, Eq)]
struct Visit {
    coord: Coord,
    /// The direction we face when entering this position
    direction: Dir,
    /// The cost to get to this position
    cost: u32,
    /// Estimated cost, for A*
    estimated_cost: u32,
}

impl Ord for Visit {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering, so that the smallest cost is at the top
        // (making this a min-heap)
        other.estimated_cost.cmp(&self.estimated_cost)
    }
}

impl PartialOrd for Visit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Run the A* algorithm to find the shortest paths from the start
/// to the end node of the board, subject to costs:
/// - Moving straight is a cost of 1
/// - Rotating left or right is a cost of 1000
pub fn run_astar(board: &Board<Cell>) -> Option<u32> {
    let start = board.find(&Cell::Start)[0];
    let end = board.find(&Cell::End)[0];

    // A* algorithm, where instead of just considering the cost of translation
    // we also consider the cost of rotation. Very similar to 2023 day 17.

    // Minimum cost of getting to a position in a given direction
    let mut costs: HashMap<(Coord, Dir), u32> = HashMap::new();

    // Locations and directions we've already visited
    let mut visited: HashSet<(Coord, Dir)> = HashSet::new();

    // Frontier, ordered by minimum cost
    let mut to_visit: BinaryHeap<Visit> = BinaryHeap::new();

    // Starting position and direction
    to_visit.push(Visit {
        coord: start,
        direction: Dir::East,
        cost: 0,
        estimated_cost: 0,
    });

    let mut iterations = 0;

    while let Some(Visit {
        coord,
        direction,
        cost,
        ..
    }) = to_visit.pop()
    {
        if matches!(board.get(&coord), Some(Cell::Wall)) {
            // Can't move here
            continue;
        }

        if !visited.insert((coord, direction)) {
            // Already been here
            continue;
        }

        iterations += 1;

        if coord == end {
            println!("Completed in {} iterations", iterations);
            return Some(cost);
        }

        // Movement possibilities, and the costs they incur
        let options = [
            // Moving in the current direction costs 1
            (coord + direction, direction, cost + 1),
            // Rotating left or right in-place costs 1000
            (coord, direction.rotate_right(), cost + 1000),
            (coord, direction.rotate_left(), cost + 1000),
        ];

        for (new_position, new_direction, new_cost) in options {
            let heuristic_cost = heuristic((new_position, new_direction), end);

            let is_cheaper = costs
                .get(&(new_position, new_direction))
                .map_or(true, |&current| new_cost < current);

            if is_cheaper {
                costs.insert((new_position, new_direction), new_cost);
                to_visit.push(Visit {
                    coord: new_position,
                    direction: new_direction,
                    cost: new_cost,
                    // Set estimated_cost: new_cost and this becomes Dijkstra's algorithm
                    estimated_cost: new_cost + heuristic_cost,
                });
            }
        }
    }

    None
}

pub fn solution(input: &str) -> u32 {
    let board: Board<Cell> = Board::transform_from_str(input, |c| match c {
        'S' => Cell::Start,
        '.' => Cell::Empty,
        '#' => Cell::Wall,
        'E' => Cell::End,
        _ => panic!("Unrecognized character {}", c),
    });

    let cost = run_astar(&board).expect("No solution found");

    return cost;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../example.txt");
        let res = solution(input);

        assert_eq!(res, 7036);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, 160624);
    }
}
