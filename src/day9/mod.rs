use std::{fmt, ops::Sub, str::FromStr, string::ParseError};

use itertools::Itertools;

use crate::tooling::SolutionResult;

#[derive(Clone, Copy)]
enum Move {
    Right(u8),
    Left(u8),
    Down(u8),
    Up(u8),
}

impl Move {
    fn get_distance(self) -> u8 {
        match self {
            Move::Right(dist) => dist,
            Move::Left(dist) => dist,
            Move::Up(dist) => dist,
            Move::Down(dist) => dist,
        }
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (name, dist) = match self {
            Move::Right(dist) => ("Right", dist),
            Move::Left(dist) => ("Left", dist),
            Move::Up(dist) => ("Up", dist),
            Move::Down(dist) => ("Down", dist),
        };
        write!(f, "{0}: {1}", name, dist)
    }
}

impl FromStr for Move {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cmd = s.split_at(1);
        let cmd = (cmd.0, cmd.1.trim().parse::<u8>().unwrap());

        Ok(match cmd.0 {
            "R" => Move::Right(cmd.1),
            "L" => Move::Left(cmd.1),
            "U" => Move::Up(cmd.1),
            "D" => Move::Down(cmd.1),
            _ => panic!("Wrong command format"),
        })
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Position {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Position {
    fn tuple(self) -> (i32, i32) {
        (self.x, self.y)
    }

    fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }
}

#[derive(Clone, Copy, Debug)]
struct Rope {
    head: Position,
    tail: Position,
}

fn build_rope_movement<F>(mut r: Rope, move_head: F, d: u8) -> impl Iterator<Item = Rope> + Clone
where
    F: Fn(Position) -> Position + Clone,
{
    let moves = (0..d).map(move |_| {
        r.head = move_head(r.head);

        match (r.head - r.tail).tuple() {
            (-1..=1, -1..=1) => (),
            (x @ (2 | -2), y @ -1..=1) => {
                r.tail.y += y;
                r.tail.x += x / 2;
            }
            (x @ -1..=1, y @ (-2 | 2)) => {
                r.tail.y += y / 2;
                r.tail.x += x;
            }
            _ => panic!("build_rope_movement unexpected head-tail difference"),
        }

        r
    });
    moves
}

impl Rope {
    fn init() -> Rope {
        Rope {
            head: Position::new(0, 0),
            tail: Position::new(0, 0),
        }
    }

    fn movement(self, m: Move) -> impl Iterator<Item = Rope> + Clone {
        let dist = m.get_distance();
        let f = match m {
            Move::Right(_) => |p: Position| Position { x: p.x + 1, y: p.y },
            Move::Left(_) => |p: Position| Position { x: p.x - 1, y: p.y },
            Move::Up(_) => |p: Position| Position { y: p.y + 1, x: p.x },
            Move::Down(_) => |p: Position| Position { y: p.y - 1, x: p.x },
        };

        build_rope_movement(self, f, dist)
    }
}

pub fn task1(input: &str) -> SolutionResult {
    let mut rope: Rope = Rope::init();

    let res = input
        .lines()
        .map(|l| l.parse::<Move>().unwrap())
        .flat_map(|m| {
            let hist = rope.movement(m);
            rope = hist.clone().last().unwrap();
            hist
        })
        //.inspect(|r| println!("Head: {0}, Tail: {1}", r.head, r.tail))
        .unique_by(|r| r.tail)
        .count();

    SolutionResult::Unsigned(res)
}

pub fn task2(input: &str) -> SolutionResult {
    SolutionResult::Unsigned(0)
}
