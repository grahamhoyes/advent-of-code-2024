use num::Integer;
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use std::ops::{Add, Mul, Sub};

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

    /// Wrap the coordinate to the given size / board dimension.
    ///
    /// Return coordinate values will always be non-negative.
    ///
    /// # Examples
    /// ```
    /// let size = (4usize, 4usize);
    /// let c = Coord(7, -5);
    /// assert!(c.wrap_to_size(size) == Coord(3, 3));
    /// ```
    pub fn wrap_to_size<T>(self, size: T) -> Coord
    where
        T: Into<Coord>,
    {
        let size = size.into();

        let row = (self.0 % size.0 + size.0) % size.0;
        let col = (self.1 % size.1 + size.1) % size.1;

        return (row, col).into();
    }

    /// Compute the manhattan distance between two coordinates
    pub fn manhattan_distance(&self, other: &Coord) -> u32 {
        (self.0.abs_diff(other.0) + self.1.abs_diff(other.1)) as u32
    }

    /// Compute the L1-norm of the coordinate vector
    ///
    /// The L1-norm is the sum of the absolute values of the components.
    pub fn l1_norm(&self) -> u32 {
        (self.0.abs() + self.1.abs()) as u32
    }

    /// Compute the L2-norm of the coordinate vector
    pub fn l2_norm(&self) -> f64 {
        ((self.0 * self.0 + self.1 * self.1) as f64).sqrt()
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

impl Mul<i32> for Coord {
    type Output = Coord;

    fn mul(self, rhs: i32) -> Self::Output {
        Coord(self.0 * rhs, self.1 * rhs)
    }
}

impl Mul<i32> for &Coord {
    type Output = Coord;

    fn mul(self, rhs: i32) -> Self::Output {
        Coord(self.0 * rhs, self.1 * rhs)
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy, Ord, PartialOrd)]
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

    /// Rotate the direction 180 degrees
    pub fn rotate_180(self) -> Self {
        match self {
            Dir::North => Dir::South,
            Dir::NorthEast => Dir::SouthWest,
            Dir::East => Dir::West,
            Dir::SouthEast => Dir::NorthWest,
            Dir::South => Dir::North,
            Dir::SouthWest => Dir::NorthEast,
            Dir::West => Dir::East,
            Dir::NorthWest => Dir::SouthEast,
        }
    }

    /// Convert the direction to degrees, between 0 and 359
    pub fn to_degrees(self) -> u32 {
        match self {
            Dir::North => 0,
            Dir::NorthEast => 45,
            Dir::East => 90,
            Dir::SouthEast => 135,
            Dir::South => 180,
            Dir::SouthWest => 225,
            Dir::West => 270,
            Dir::NorthWest => 315,
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

    /// Get the degree offset between two directions, measuring the rotation needed
    /// to get from the reference direction (other) to this direction.
    /// Positive means clockwise rotation, negative means counter-clockwise.
    ///
    /// # Examples
    /// ```
    /// use grid_2d::Dir;
    /// let north = Dir::North;
    /// let east = Dir::East;
    /// let north_east = Dir::NorthEast;
    /// let south_west = Dir::SouthWest;
    ///
    /// // To get from East to North requires -90 degree (counterclockwise) rotation
    /// assert!(north.offset_from(&east) == -90);
    ///
    /// // To get from North to East requires 90 degree (clockwise) rotation
    /// assert!(east.offset_from(&north) == 90);
    ///
    /// // To get from NorthEast to North requires -45 degree rotation
    /// assert!(north.offset_from(&north_east) == -45);
    ///
    /// // To get from SouthWest to NorthEast requires 180 degrees
    /// // Note: 180 is always returned instead of -180
    /// assert!(north_east.offset_from(&south_west) == 180);
    /// ```
    pub fn offset_from(&self, other: &Dir) -> i32 {
        let self_degrees = self.to_degrees() as i32;
        let other_degrees = other.to_degrees() as i32;

        let mut diff = self_degrees - other_degrees;

        // Normalize to (-180, 180]
        if diff > 180 {
            diff -= 360;
        } else if diff <= -180 {
            diff += 360;
        }

        // Special case, prefer 180 over -180
        if diff == -180 {
            180
        } else {
            diff
        }
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

/// Convert a coordinate vector to a direction
///
/// # Panics
/// Panics if the coordinate vector is not a cardinal or diagonal direction
impl From<Coord> for Dir {
    fn from(coord: Coord) -> Self {
        match coord.simplify() {
            Coord(-1, 0) => Dir::North,
            Coord(-1, 1) => Dir::NorthEast,
            Coord(0, 1) => Dir::East,
            Coord(1, 1) => Dir::SouthEast,
            Coord(1, 0) => Dir::South,
            Coord(1, -1) => Dir::SouthWest,
            Coord(0, -1) => Dir::West,
            Coord(-1, -1) => Dir::NorthWest,
            _ => panic!("Invalid direction vector"),
        }
    }
}

#[derive(Debug, Clone)]
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

    /// Construct a board with the given size, and fill all elements with the
    /// given item
    pub fn from_size<S>(size: S, item: T) -> Self
    where
        S: Into<Coord>,
    {
        let size = size.into();

        let matrix: Vec<Vec<T>> = (0..size.0)
            .map(|_x| (0..size.1).map(|_y| item.clone()).collect())
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

    /// Construct a vector of all coordinate positions on the board
    pub fn positions(&self) -> Vec<Coord> {
        (0..self.matrix.len())
            .flat_map(|row| (0..self.matrix[row].len()).map(move |col| (row, col).into()))
            .collect()
    }

    /// Print the board to the terminal
    pub fn print(&self)
    where
        T: Display,
    {
        for row in self.matrix.iter() {
            for item in row.iter() {
                print!("{}", item);
            }
            println!();
        }
    }

    /// Print the board with axes numbers
    pub fn print_with_axes(&self)
    where
        T: Display,
    {
        let (rows, cols) = self.size();
        let row_space = (rows - 1).to_string().len();
        let col_space = (cols - 1).to_string().len();

        let row_labels: Vec<String> = (0..rows)
            // There's intentionally an extra space here
            .map(|i| format!("{:0>width$} ", i, width = row_space))
            .collect();
        let col_labels: Vec<String> = (0..cols)
            .map(|i| format!("{:0>width$}", i, width = col_space))
            .collect();

        // Print the labels for the columns along the top first
        for i in 0..col_space {
            // Buffer room for row labels, including an extra space
            for _ in 0..=row_space {
                print!(" ")
            }

            for j in 0..cols {
                let e = &col_labels[j][i..=i];
                print!("{}", e);
            }
            println!();
        }

        for (i, row) in self.matrix.iter().enumerate() {
            // Print the row labels
            print!("{}", row_labels[i]);

            // Print the actual grid items
            for item in row.iter() {
                print!("{}", item);
            }

            println!();
        }
    }
}

impl Board<char> {
    pub fn from_str(input: &str) -> Self {
        let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        Self::new(matrix)
    }
}
