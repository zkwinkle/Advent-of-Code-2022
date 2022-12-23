use std::{
    fmt,
    ops::{Add, Sub},
};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Position<T> {
    pub x: T,
    pub y: T,
}

impl<T: fmt::Display> fmt::Display for Position<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T: Sub<Output = T>> Sub for Position<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Position {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T: Add<Output = T>> Add for Position<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> Position<T> {
    pub fn new(x: T, y: T) -> Position<T> { Position { x, y } }
}

impl<T> From<(T, T)> for Position<T> {
    fn from(tuple: (T, T)) -> Position<T> { Position::new(tuple.0, tuple.1) }
}

impl<T> From<Position<T>> for (T, T) {
    fn from(p: Position<T>) -> (T, T) { (p.x, p.y) }
}
