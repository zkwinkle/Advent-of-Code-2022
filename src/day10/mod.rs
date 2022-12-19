use std::{fmt, str::FromStr, string::ParseError};

use itertools::Either;

use aoc_lib::tooling::SolutionResult;

#[derive(Clone, Copy)]
enum Instruction {
    Noop,
    Addx(i32),
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Noop => write!(f, "noop"),
            Instruction::Addx(add) => write!(f, "addx({add})"),
        }
    }
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match &s[0..4] {
            "noop" => Instruction::Noop,
            "addx" => Instruction::Addx(s[5..].parse().unwrap()),
            _ => panic!("Wrong instruction format"),
        })
    }
}

fn generate_cycles(iter: impl Iterator<Item = Instruction>) -> impl Iterator<Item = i32> {
    let mut x_reg = 1;

    (1..=1).chain(
        iter //.inspect(|inst| println!("{inst}"))
            .flat_map(move |inst| match inst {
                Instruction::Noop => Either::Left([x_reg].into_iter()),
                Instruction::Addx(add) => {
                    x_reg += add;
                    Either::Right([x_reg - add, x_reg].into_iter())
                }
            }),
    )
    //.inspect(|x| println!("X: {x}"))
}

pub fn task1(input: &str) -> SolutionResult {
    let cycles = generate_cycles(input.lines().map(|l| l.parse().unwrap()));

    let res = cycles
        .enumerate()
        .skip(19)
        .step_by(40)
        //.inspect(|(i, x)| println!("{}: X({x}) => {}", i + 1, (i + 1) as i32 * x))
        .fold(0, |acc, (i, x)| acc + (i + 1) as i32 * x);

    SolutionResult::Signed(res)
}

#[allow(dead_code)]
fn b2c(b: bool) -> char {
    match b {
        true => '#',
        false => '.',
    }
}

pub fn task2(input: &str) -> SolutionResult {
    let cycles = generate_cycles(input.lines().map(|l| l.parse().unwrap())).take(240);

    for (i, x) in cycles.enumerate() {
        let pos = i % 40;

        #[allow(unused_variables)]
        let pixel = (x - 1..=x + 1).contains(&(pos as i32));

        //print!("{}", b2c(pixel));

        if pos == 39 {
            //print!("\n");
        }
    }

    SolutionResult::Str("Enable prints to see result".to_string())
}
