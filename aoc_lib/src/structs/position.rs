use std::{
    fmt,
    ops::{Add, Sub},
};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Position<T> {
    x: T,
    y: T,
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
    fn tuple(self) -> (T, T) { (self.x, self.y) }

    fn new(x: T, y: T) -> Position<T> { Position { x, y } }
}
