use std::fmt;

pub enum SolutionResult {
    Str(String),
    Signed(i32),
    Unsigned(usize),
}

impl fmt::Display for SolutionResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SolutionResult::Str(str) => write!(f, "{}", str),
            SolutionResult::Signed(n) => write!(f, "{}", n),
            SolutionResult::Unsigned(n) => write!(f, "{}", n),
        }
    }
}

pub type Solution = fn(&str) -> SolutionResult;

macro_rules! solutions {
    ($max_day:expr) => {
            seq!(N in 1..=$max_day {
            [
            #([
                |data: _| day~N::task1(data),
                |data: _| day~N::task2(data)
            ],)*
        ]
            })
    };
}

macro_rules! inputs {
    ($max_day:expr) => {
            seq!(N in 1..=$max_day {
            [
            #(
                    [
             include_str!(concat!("day", N, "/input.txt")),
             include_str!(concat!("day", N, "/testinput.txt")),
                    ]
            ,)*
        ]
            })
    };
}
