use std::{cmp::Ordering, fmt, str::FromStr, string::ParseError};

use aoc_lib::tooling::SolutionResult;

#[derive(Debug, Clone, PartialEq, Eq)]
enum PacketData {
    List(Vec<PacketData>),
    Integer(u32),
}

impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PacketData {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (PacketData::List(l), i @ PacketData::Integer(_)) => {
                l.cmp(&[i.clone()].into())
            }
            (i @ PacketData::Integer(_), PacketData::List(l)) => {
                vec![i.clone()].cmp(l)
            }

            (PacketData::List(l1), PacketData::List(l2)) => l1.cmp(l2),
            (PacketData::Integer(i1), PacketData::Integer(i2)) => i1.cmp(i2),
        }
    }
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

    let mut res = 0;
    for (i, pair) in pairs.iter().enumerate() {
        //println!(
        //    "The lists are in the {} order",
        //    if pair[0] < pair[1] { "right" } else { "wrong" }
        //);
        //println!("{}\n{}\n", pair[0], pair[1]);
        if pair[0] < pair[1] {
            res += i + 1
        }
    }

    SolutionResult::Unsigned(res)
}

pub fn task2(input: &str) -> SolutionResult { SolutionResult::Unsigned(0) }
