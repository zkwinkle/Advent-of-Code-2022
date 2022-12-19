use std::{fmt, ops::Sub, str::FromStr, string::ParseError};

use itertools::Itertools;

use aoc_lib::tooling::SolutionResult;

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
        write!(f, "{name}: {dist}")
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
struct Rope<const N: usize> {
    knots: [Position; N],
}

impl<const N: usize> fmt::Display for Rope<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\tHEAD:\t{}", self.head())?;
        for i in 1..N - 1 {
            writeln!(f, "\t{i}:\t{}", self.knots[i])?;
        }
        write!(f, "\tTAIL:\t{}", self.tail())
    }
}

fn build_rope_movement<F, const N: usize>(
    mut r: Rope<N>,
    move_head: F,
    d: u8,
) -> impl Iterator<Item = Rope<N>> + Clone
where
    F: Fn(Position) -> Position + Clone,
{
    (0..d).map(move |_| {
        r.knots[0] = move_head(r.head());

        for (i_h, i_t) in (0..N - 1).zip(1..N) {
            match (r.knots[i_h] - r.knots[i_t]).tuple() {
                (-1..=1, -1..=1) => (),
                (x @ (2 | -2), y @ -1..=1) => {
                    r.knots[i_t].y += y;
                    r.knots[i_t].x += x / 2;
                }
                (x @ -1..=1, y @ (-2 | 2)) => {
                    r.knots[i_t].y += y / 2;
                    r.knots[i_t].x += x;
                }
                (x @ (-2 | 2), y @ (-2 | 2)) => {
                    r.knots[i_t].y += y / 2;
                    r.knots[i_t].x += x / 2;
                }
                _ => {
                    panic!("build_rope_movement unexpected head-tail difference")
                }
            }
        }

        r
    })
}

impl<const N: usize> Rope<N> {
    fn init() -> Rope<N> {
        Rope {
            knots: [Position::new(0, 0); N],
        }
    }

    fn head(&self) -> Position {
        self.knots[0]
    }

    fn tail(&self) -> Position {
        self.knots[N - 1]
    }

    fn movement(self, m: Move) -> impl Iterator<Item = Rope<N>> + Clone {
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

pub fn solve<const N: usize>(input: &str) -> usize {
    let lines = input.lines();
    let moves = lines.clone().map(|l| l.parse::<Move>().unwrap());

    let hist_size = lines.count() * 5; // approximation

    let mut history: Vec<Rope<N>> = Vec::with_capacity(hist_size);
    history.push(Rope::init());

    for m in moves {
        history.extend(history.last().unwrap().movement(m));
        //println!("\n{m}\n{}", history.last().unwrap());
    }

    history
        .iter()
        //.inspect(|r| println!("Head: {0}, Tail: {1}", r.head, r.tail))
        .unique_by(|r| r.tail())
        .count()
}

pub fn task1(input: &str) -> SolutionResult {
    SolutionResult::Unsigned(solve::<2>(input))
}

pub fn task2(input: &str) -> SolutionResult {
    SolutionResult::Unsigned(solve::<10>(input))
}
