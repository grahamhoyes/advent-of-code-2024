use num::Integer;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::{Add, Sub};

/// A (row, col) coordinate pair or vector. Using i32 so that we can subtract
/// or have negative vectors.
#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub struct Coord(pub i32, pub i32);

impl Coord {
    /// Simplify the coordinate vector by dividing both components by their
    /// greatest common divisor.
    ///
    /// # Examples
    /// ```
    /// let c = Coord(4, 6);
    /// assert!(c.simplify() == Coord(2, 3));
    /// ```
    pub fn simplify(&self) -> Coord {
        let gcd = self.0.gcd(&self.1);

        Coord(self.0 / gcd, self.1 / gcd)
    }

    /// Get the neighbours of a coordinate in the cardinal directions.
    ///
    /// Sorted clockwise starting from the north.
    pub fn cardinal_neighbours(&self) -> [Coord; 4] {
        [
            self + Dir::North,
            self + Dir::East,
            self + Dir::South,
            self + Dir::West,
        ]
    }
}

impl From<Coord> for (i32, i32) {
    fn from(value: Coord) -> Self {
        (value.0, value.1)
    }
}

impl From<(i32, i32)> for Coord {
    fn from(value: (i32, i32)) -> Self {
        Coord(value.0, value.1)
    }
}

impl From<(usize, usize)> for Coord {
    fn from(value: (usize, usize)) -> Self {
        Coord(value.0 as i32, value.1 as i32)
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum Dir {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Dir {
    /// Rotate the direction 90 degrees clockwise
    pub fn rotate_right(self) -> Self {
        match self {
            Dir::North => Dir::East,
            Dir::NorthEast => Dir::SouthEast,
            Dir::East => Dir::South,
            Dir::SouthEast => Dir::SouthWest,
            Dir::South => Dir::West,
            Dir::SouthWest => Dir::NorthWest,
            Dir::West => Dir::North,
            Dir::NorthWest => Dir::NorthEast,
        }
    }

    /// Rotate the direction 90 degrees counter-clockwise
    pub fn rotate_left(self) -> Self {
        match self {
            Dir::North => Dir::West,
            Dir::NorthEast => Dir::NorthWest,
            Dir::East => Dir::North,
            Dir::SouthEast => Dir::NorthEast,
            Dir::South => Dir::East,
            Dir::SouthWest => Dir::SouthEast,
            Dir::West => Dir::South,
            Dir::NorthWest => Dir::SouthWest,
        }
    }

    /// Rotate the direction 45 degrees clockwise
    pub fn rotate_right_45(self) -> Self {
        match self {
            Dir::North => Dir::NorthEast,
            Dir::NorthEast => Dir::East,
            Dir::East => Dir::SouthEast,
            Dir::SouthEast => Dir::South,
            Dir::South => Dir::SouthWest,
            Dir::SouthWest => Dir::West,
            Dir::West => Dir::NorthWest,
            Dir::NorthWest => Dir::North,
        }
    }

    /// Rotate the direction 45 degrees counter-clockwise
    pub fn rotate_left_45(self) -> Self {
        match self {
            Dir::North => Dir::NorthWest,
            Dir::NorthEast => Dir::North,
            Dir::East => Dir::NorthEast,
            Dir::SouthEast => Dir::East,
            Dir::South => Dir::SouthEast,
            Dir::SouthWest => Dir::South,
            Dir::West => Dir::SouthWest,
            Dir::NorthWest => Dir::West,
        }
    }

    /// Get all directions except the one that is the opposite of this direction
    ///
    /// Only defined for the cardinal directions.
    pub fn not_backwards(self) -> Vec<Self> {
        match self {
            Dir::North => vec![Dir::North, Dir::East, Dir::West],
            Dir::East => vec![Dir::North, Dir::East, Dir::South],
            Dir::South => vec![Dir::East, Dir::South, Dir::West],
            Dir::West => vec![Dir::North, Dir::South, Dir::West],
            _ => unimplemented!("Only defined for cardinal directions"),
        }
    }

    pub fn cardinal() -> [Self; 4] {
        [Dir::North, Dir::East, Dir::South, Dir::West]
    }

    pub fn all() -> [Self; 8] {
        [
            Dir::North,
            Dir::NorthEast,
            Dir::East,
            Dir::SouthEast,
            Dir::South,
            Dir::SouthWest,
            Dir::West,
            Dir::NorthWest,
        ]
    }
}

impl Add<Dir> for Coord {
    type Output = Coord;

    fn add(self, rhs: Dir) -> Self::Output {
        (&self).add(rhs)
    }
}

impl Add<Dir> for &Coord {
    type Output = Coord;

    fn add(self, rhs: Dir) -> Self::Output {
        match rhs {
            Dir::North => Coord(self.0 - 1, self.1),
            Dir::NorthEast => Coord(self.0 - 1, self.1 + 1),
            Dir::East => Coord(self.0, self.1 + 1),
            Dir::SouthEast => Coord(self.0 + 1, self.1 + 1),
            Dir::South => Coord(self.0 + 1, self.1),
            Dir::SouthWest => Coord(self.0 + 1, self.1 - 1),
            Dir::West => Coord(self.0, self.1 - 1),
            Dir::NorthWest => Coord(self.0 - 1, self.1 - 1),
        }
    }
}

impl Add<Coord> for Coord {
    type Output = Coord;

    fn add(self, rhs: Coord) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Add<&Coord> for &Coord {
    type Output = Coord;

    fn add(self, rhs: &Coord) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub<Coord> for Coord {
    type Output = Coord;

    fn sub(self, rhs: Coord) -> Self::Output {
        Coord(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Sub<&Coord> for &Coord {
    type Output = Coord;

    fn sub(self, rhs: &Coord) -> Self::Output {
        Coord(self.0 - rhs.0, self.1 - rhs.1)
    }
}

#[derive(Debug)]
pub struct Board<T>
where
    T: Clone,
{
    pub matrix: Vec<Vec<T>>,
}

impl<T> Board<T>
where
    T: Clone,
{
    pub fn new(matrix: Vec<Vec<T>>) -> Self {
        Self { matrix }
    }

    /// Creates a new board by transforming a string input, mapping each character to a board
    /// element using the provided transformation function.
    ///
    /// # Arguments
    /// * `input` - A string representing the board, with rows separated by newlines
    /// * `transform` - A function that converts each character to the board's element type
    ///
    /// # Examples
    /// ```
    /// use grid_2d::Board;
    ///
    /// #[derive(Debug, Clone, Hash)]
    /// enum Cell {
    ///     Empty,
    ///     Rock,
    ///     Sand,
    /// }
    ///
    /// let input =
    ///     "SR.\n\
    ///      .R.\n\
    ///      ..R";
    ///
    /// let board = Board::transform_from_str(input, |c| match c {
    ///     'S' => Cell::Sand,
    ///     'R' => Cell::Rock,
    ///     '.' => Cell::Empty,
    ///     _ => panic!("unexpected character"),
    /// });
    ///
    /// assert_eq!(board.get(&Coord(0, 0)), Some(Cell::Start));
    /// assert_eq!(board.get(&Coord(0, 1)), Some(Cell::Rock));
    /// assert_eq!(board.get(&Coord(2, 2)), Some(Cell::Rock));
    /// ```
    pub fn transform_from_str<F>(input: &str, transform: F) -> Self
    where
        F: Fn(char) -> T,
    {
        let matrix: Vec<Vec<T>> = input
            .lines()
            .map(|line| line.chars().map(|c| transform(c)).collect())
            .collect();

        Self::new(matrix)
    }

    pub fn size(&self) -> (usize, usize) {
        (self.matrix.len(), self.matrix[0].len())
    }

    pub fn get(&self, c: &Coord) -> Option<T> {
        let (rows, cols) = self.size();

        if c.0 < 0 || c.0 as usize >= rows || c.1 < 0 || c.1 as usize >= cols {
            return None;
        }

        Some(self.matrix[c.0 as usize][c.1 as usize].clone())
    }

    /// Get the value at a coordinate without checking for bounds
    ///
    /// # Panics
    /// Panics if the coordinate is outside of the board
    pub fn get_unchecked(&self, c: &Coord) -> T {
        self.get(c).unwrap()
    }

    /// Find the position of all occurrences of `elem` on the board.
    ///
    /// Returns a vector of coordinates.
    pub fn find(&self, elem: &T) -> Vec<Coord>
    where
        T: Eq,
    {
        self.matrix
            .iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter().enumerate().filter_map(move |(j, e)| {
                    if e == elem {
                        Some((i, j).into())
                    } else {
                        None
                    }
                })
            })
            .flatten()
            .collect()
    }

    pub fn set(&mut self, c: &Coord, val: T) {
        self.matrix[c.0 as usize][c.1 as usize] = val;
    }

    /// Returns a HashMap containing positions of elements that match the given filter.
    /// Elements are grouped by type, with their positions collected into a Vec<Coord>.
    ///
    /// The elements to include are determined by the provided closure `filter`.
    ///
    /// # Examples
    /// ```
    /// use grid_2d::{Board, Coord};
    ///
    /// // Using with a char board - collecting all non-empty spaces
    /// let board = Board::from_str(
    ///     "A....\n\
    ///      ..X..\n\
    ///      X..X.\n\
    ///      ....."
    /// );
    ///
    /// let positions = board.find_positions(|c| *c != '.');
    /// assert_eq!(positions.get(&'A').unwrap(), &vec![Coord(0, 0)]);
    /// assert_eq!(positions.get(&'X').unwrap().len(), 3);
    ///
    /// // Or collecting just 'X' characters
    /// let x_positions = board.find_positions(|c| *c == 'X');
    /// assert_eq!(x_positions.get(&'X').unwrap().len(), 3);
    ///
    /// // Using with an enum
    /// #[derive(Debug, Clone, Hash, Eq, PartialEq)]
    /// enum Cell {
    ///     Empty,
    ///     Rock,
    ///     Sand,
    /// }
    ///
    /// let board = Board::new(vec![
    ///     vec![Cell::Rock, Cell::Empty, Cell::Sand],
    ///     vec![Cell::Empty, Cell::Rock, Cell::Empty],
    /// ]);
    ///
    /// // Collecting all non-empty cells
    /// let positions = board.find_positions(|cell| !matches!(cell, Cell::Empty));
    /// assert_eq!(positions.get(&Cell::Rock).unwrap().len(), 2);
    /// assert_eq!(positions.get(&Cell::Sand).unwrap().len(), 1);
    /// ```
    pub fn find_positions<P>(&self, filter: P) -> HashMap<T, Vec<Coord>>
    where
        P: Fn(&T) -> bool,
        T: Clone + Hash + Eq,
    {
        let mut result = HashMap::new();

        for (i, row) in self.matrix.iter().enumerate() {
            for (j, item) in row.iter().enumerate() {
                if !filter(item) {
                    continue;
                }

                result
                    .entry(item.clone())
                    .or_insert_with(Vec::new)
                    .push((i, j).into());
            }
        }

        result
    }

    pub fn positions(&self) -> Vec<Coord> {
        (0..self.matrix.len())
            .flat_map(|row| (0..self.matrix[row].len()).map(move |col| (row, col).into()))
            .collect()
    }
}

impl Board<char> {
    pub fn from_str(input: &str) -> Self {
        let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        Self::new(matrix)
    }
}
