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
    East,
    South,
    West,
}

impl Dir {
    pub fn rotate_right(self) -> Self {
        match self {
            Dir::North => Dir::East,
            Dir::East => Dir::South,
            Dir::South => Dir::West,
            Dir::West => Dir::North,
        }
    }

    pub fn rotate_left(self) -> Self {
        match self {
            Dir::North => Dir::West,
            Dir::East => Dir::North,
            Dir::South => Dir::East,
            Dir::West => Dir::South,
        }
    }
}

impl Add<Dir> for Coord {
    type Output = Coord;

    fn add(self, rhs: Dir) -> Self::Output {
        match rhs {
            Dir::North => Coord(self.0 - 1, self.1),
            Dir::East => Coord(self.0, self.1 + 1),
            Dir::South => Coord(self.0 + 1, self.1),
            Dir::West => Coord(self.0, self.1 - 1),
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
    /// let positions = board.element_positions(|c| *c != '.');
    /// assert_eq!(positions.get(&'A').unwrap(), &vec![Coord(0, 0)]);
    /// assert_eq!(positions.get(&'X').unwrap().len(), 3);
    ///
    /// // Or collecting just 'X' characters
    /// let x_positions = board.element_positions(|c| *c == 'X');
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
    /// let positions = board.element_positions(|cell| !matches!(cell, Cell::Empty));
    /// assert_eq!(positions.get(&Cell::Rock).unwrap().len(), 2);
    /// assert_eq!(positions.get(&Cell::Sand).unwrap().len(), 1);
    /// ```
    pub fn element_positions<P>(&self, filter: P) -> HashMap<T, Vec<Coord>>
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
}

impl Board<char> {
    pub fn from_str(input: &str) -> Self {
        let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        Self::new(matrix)
    }
}
