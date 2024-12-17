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
pub fn heuristic(current: &State, target: &Coord) -> u32 {
    let direction_vec = target - &current.position;

    // Whichever coordinate has a greater magnitude can indicate roughly which
    // cardinal direction we should try to move in.
    // Since the end is always in the top right, the row coordinate will always
    // be <= 0 and the column >=0
    let (d_rows, d_cols) = (direction_vec.0.abs(), direction_vec.1.abs());
    let target_dir = match d_rows.cmp(&d_cols) {
        Ordering::Greater => Dir::North,   // More rows to move than columns
        Ordering::Less => Dir::East,       // More columns to move than rows
        Ordering::Equal => current.facing, // Doesn't matter, stay current
    };

    let rotations_required = (current.facing.offset_from(&target_dir) / 90).unsigned_abs();

    direction_vec.l1_norm() + 1000 * rotations_required
}

/// A visit to a state in the A* algorithm
///
/// `estimated_cost` is the heuristic-based cost estimate from this state
/// to the end state, which is used to prioritize the frontier.
#[derive(Debug, PartialEq, Eq)]
struct Visit {
    state: State,
    /// The cost to get to this position
    current_cost: u32,
    /// Heuristic-estimated cost to the end state, for A*
    estimated_total_cost: u32,
}

impl Ord for Visit {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering, so that the smallest cost is at the top
        // (making this a min-heap)
        other.estimated_total_cost.cmp(&self.estimated_total_cost)
    }
}

impl PartialOrd for Visit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// The fundamental state of the actor on the board at any given time
/// for this problem has to consider both position and direction,
/// since rotations incur a cost.
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct State {
    pub position: Coord,
    pub facing: Dir,
}

/// Map from a state to its parents.
///
/// The parents of a state are the states that can lead to it,
/// all of which have an equal and lowest cost.
pub type Parents = HashMap<State, HashSet<State>>;

/// Run the A* algorithm to find the shortest paths from the start
/// to the end node of the board, subject to costs:
/// - Moving straight is a cost of 1
/// - Rotating left or right is a cost of 1000
pub fn run_astar(board: &Board<Cell>) -> Option<(u32, Parents)> {
    let start = board.find(&Cell::Start)[0];
    let end = board.find(&Cell::End)[0];

    // A* algorithm, where instead of just considering the cost of translation
    // we also consider the cost of rotation. Very similar to 2023 day 17.

    // Minimum cost of getting to a position in a given direction
    let mut costs: HashMap<State, u32> = HashMap::new();
    // Locations and directions we've already visited
    let mut visited: HashSet<State> = HashSet::new();
    // Frontier, ordered by minimum cost
    let mut to_visit: BinaryHeap<Visit> = BinaryHeap::new();
    // Parents, which can construct the optimal paths through the graph
    let mut parents: Parents = HashMap::new();

    // Starting position and direction
    to_visit.push(Visit {
        state: State {
            position: start,
            facing: Dir::East, // Starting East given in the problem definition
        },
        current_cost: 0,
        estimated_total_cost: 0,
    });

    let mut iterations = 0;

    while let Some(Visit {
        state,
        current_cost: cost,
        ..
    }) = to_visit.pop()
    {
        if matches!(board.get(&state.position), Some(Cell::Wall)) {
            // Can't move here
            continue;
        }

        if !visited.insert(state.clone()) {
            // Already been here
            continue;
        }

        iterations += 1;

        if state.position == end {
            println!("Completed in {} iterations", iterations);
            return Some((cost, parents));
        }

        // Movement possibilities, and the costs they incur
        let options = [
            // Moving in the current direction costs 1
            (state.position + state.facing, state.facing, cost + 1),
            // Rotating left or right in-place costs 1000
            (state.position, state.facing.rotate_right(), cost + 1000),
            (state.position, state.facing.rotate_left(), cost + 1000),
        ];

        for (new_position, new_direction, new_cost) in options {
            let new_state = State {
                position: new_position,
                facing: new_direction,
            };

            let heuristic_cost = heuristic(&new_state, &end);

            let cost_comparison = costs
                .get(&new_state)
                .map_or(Ordering::Less, |&current| new_cost.cmp(&current));

            match cost_comparison {
                Ordering::Less => {
                    costs.insert(new_state.clone(), new_cost);
                    to_visit.push(Visit {
                        state: new_state.clone(),
                        current_cost: new_cost,
                        // Set estimated_cost: new_cost and this becomes Dijkstra's algorithm
                        estimated_total_cost: new_cost + heuristic_cost,
                    });
                    // When a new cheapest path is found, reset the parents of this node
                    parents.insert(new_state, HashSet::from([state.clone()]));
                }
                Ordering::Equal => {
                    // If the cost is the same, we can add this to the parents
                    // of the current state
                    parents.entry(new_state).or_default().insert(state.clone());
                }
                _ => {}
            }
        }
    }

    None
}

pub fn parse_input(input: &str) -> Board<Cell> {
    Board::transform_from_str(input, |c| match c {
        'S' => Cell::Start,
        '.' => Cell::Empty,
        '#' => Cell::Wall,
        'E' => Cell::End,
        _ => panic!("Unrecognized character {}", c),
    })
}

pub fn solution(input: &str) -> u32 {
    let board = parse_input(input);

    let (cost, _parents) = run_astar(&board).expect("No solution found");

    cost
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
