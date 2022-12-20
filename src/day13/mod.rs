use std::{fmt, mem, str::FromStr, string::ParseError};

use aoc_lib::tooling::SolutionResult;

#[derive(Debug)]
enum PacketData {
    List(Vec<PacketData>),
    Integer(u32),
}

impl fmt::Display for PacketData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PacketData::List(vec) => {
                write!(f, "[")?;
                for (i, p) in vec.iter().enumerate() {
                    write!(f, "{}", p)?;
                    if i < vec.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            }
            PacketData::Integer(i) => write!(f, "{}", i),
        }
    }
}

impl FromStr for PacketData {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().next().unwrap() == '[' {
            let mut list = Vec::with_capacity(16);

            let mut chars = s.char_indices();

            chars.next(); // get rid of initial '['

            'outer: while let Some((i, c)) = chars.next() {
                if c == ',' {
                    continue;
                }
                if c == '[' {
                    let mut nesting = 0;
                    while let Some((end_i, c)) = chars.next() {
                        if c == '[' {
                            nesting += 1;
                        } else if c == ']' {
                            if nesting == 0 {
                                list.push(s[i..=end_i].parse().unwrap());
                                break;
                            } else {
                                nesting -= 1;
                            }
                        }
                    }
                } else {
                    while let Some((end_i, c)) = chars.next() {
                        if c == ',' || c == ']' {
                            list.push(s[i..end_i].parse().unwrap());
                            if c == ']' {
                                break 'outer;
                            }
                            break;
                        }
                    }
                }
            }

            Ok(PacketData::List(list))
        } else {
            Ok(PacketData::Integer(s.trim_matches(',').parse().unwrap()))
        }
    }
}

fn parse(input: &str) -> Vec<[PacketData; 2]> {
    input
        .split("\n\n")
        .map(|lines: &str| {
            let l = lines.split_once('\n').unwrap();
            [l.0.parse().unwrap(), l.1.parse().unwrap()]
        })
        .collect()
}

pub fn task1(input: &str) -> SolutionResult {
    let pairs: Vec<[PacketData; 2]> = parse(input);
    //for pair in pairs {
    //    println!("{}\n{}\n", pair[0], pair[1]);
    //}

    SolutionResult::Unsigned(0)
}

pub fn task2(input: &str) -> SolutionResult { SolutionResult::Unsigned(0) }
