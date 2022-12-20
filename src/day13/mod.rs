use std::{cmp::Ordering, fmt, str::FromStr, string::ParseError};

use aoc_lib::tooling::SolutionResult;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    List(Vec<Packet>),
    Integer(u32),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::List(l), i @ Packet::Integer(_)) => {
                l.cmp(&[i.clone()].into())
            }
            (i @ Packet::Integer(_), Packet::List(l)) => vec![i.clone()].cmp(l),

            (Packet::List(l1), Packet::List(l2)) => l1.cmp(l2),
            (Packet::Integer(i1), Packet::Integer(i2)) => i1.cmp(i2),
        }
    }
}

impl fmt::Display for Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Packet::List(vec) => {
                write!(f, "[")?;
                for (i, p) in vec.iter().enumerate() {
                    write!(f, "{}", p)?;
                    if i < vec.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            }
            Packet::Integer(i) => write!(f, "{}", i),
        }
    }
}

impl FromStr for Packet {
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

            Ok(Packet::List(list))
        } else {
            Ok(Packet::Integer(s.trim_matches(',').parse().unwrap()))
        }
    }
}

pub fn task1(input: &str) -> SolutionResult {
    let pairs: Vec<[Packet; 2]> = input
        .split("\n\n")
        .map(|lines: &str| {
            let l = lines.split_once('\n').unwrap();
            [l.0.parse().unwrap(), l.1.parse().unwrap()]
        })
        .collect();

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

pub fn task2(input: &str) -> SolutionResult {
    let mut packets: Vec<Packet> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l: &str| l.parse().unwrap())
        .collect();

    // Divider packets
    let divider_2 = Packet::List(vec![Packet::List(vec![Packet::Integer(2)])]);
    let divider_6 = Packet::List(vec![Packet::List(vec![Packet::Integer(6)])]);
    packets.push(divider_2.clone());
    packets.push(divider_6.clone());

    packets.sort();

    let mut res = 1;
    for (i, packet) in packets.iter().enumerate() {
        if *packet == divider_2 || *packet == divider_6 {
            res *= i + 1;
        }
    }

    SolutionResult::Unsigned(res)
}
