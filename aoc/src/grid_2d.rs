use std::ops::Add;

/// A (row, col) coordinate pair or vector. Using i32 so that we can subtract
/// or have negative vectors.
#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub struct Coord(pub i32, pub i32);

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

#[derive(Debug)]
pub struct Board<T>
where
    T: Clone,
{
    pub inner: Vec<Vec<T>>,
}

impl<T> Board<T>
where
    T: Clone,
{
    pub fn new(board: Vec<Vec<T>>) -> Self {
        Self { inner: board }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.inner.len(), self.inner[0].len())
    }

    pub fn get(&self, c: &Coord) -> Option<T> {
        let (rows, cols) = self.size();

        if c.0 < 0 || c.0 as usize >= rows || c.1 < 0 || c.1 as usize >= cols {
            return None;
        }

        Some(self.inner[c.0 as usize][c.1 as usize].clone())
    }

    pub fn set(&mut self, c: &Coord, val: T) {
        self.inner[c.0 as usize][c.1 as usize] = val;
    }
}
