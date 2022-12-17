use std::{
    cmp::Ordering,
    collections::VecDeque,
    fmt::Debug,
    ops::{Add, Div, Mul},
    str::FromStr,
    string::ParseError,
};

use crate::tooling::SolutionResult;

type Num = u32;

#[derive(Debug)]
enum Op<M: Mul, A: Add> {
    Sum(A),
    Mul(M),
    Square,
}

#[derive(Debug)]
struct Monkey<T: Div + Mul + Add> {
    id: usize,
    items: VecDeque<T>,
    operation: Op<T, T>,

    /// Tuple: (factor, who to pass if divisible, who to pass if not divisible)
    div_test: (T, usize, usize),

    inspections: usize,
}

impl<T> FromStr for Monkey<T>
where
    T: Div + Mul + Add + FromStr<Err: Debug>,
{
    type Err = ParseError;

    /// Assumes &str starts with 6 lines with the format:
    /// Monkey #:
    ///  Starting items: #, #, ...
    ///  Operation: new = old * #
    ///  Test: divisible by #
    ///    If true: throw to monkey #
    ///    If false: throw to monkey #
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let id: usize = lines
            .next()
            .unwrap()
            .trim_start_matches("Monkey ")
            .trim_end_matches(':')
            .parse()
            .unwrap();

        let items = lines
            .next()
            .unwrap()
            .trim_start()
            .trim_start_matches("Starting items: ")
            .split(',')
            .map(|n| n.trim().parse::<T>().unwrap())
            .collect();

        let mut operation = lines
            .next()
            .unwrap()
            .trim_start()
            .trim_start_matches("Operation: new = old ")
            .split_ascii_whitespace();

        let operation = match (operation.next().unwrap(), operation.next().unwrap()) {
            ("*", "old") => Op::Square,
            ("*", num) => Op::Mul(num.trim().parse().unwrap()),
            ("+", num) => Op::Sum(num.trim().parse().unwrap()),
            (_, _) => panic!("Unexpected operation"),
        };

        let divisor = lines
            .next()
            .unwrap()
            .trim_start()
            .trim_start_matches("Test: divisible by ")
            .parse()
            .expect("Wrong Monkey divisor format");

        let if_true = lines
            .next()
            .unwrap()
            .trim_start()
            .trim_start_matches("If true: throw to monkey ")
            .parse()
            .expect("Wrong Monkey If true condition");
        let if_false = lines
            .next()
            .unwrap()
            .trim_start()
            .trim_start_matches("If false: throw to monkey ")
            .parse()
            .expect("Wrong Monkey If false condition");

        Ok(Monkey {
            id,
            items,
            operation,
            div_test: (divisor, if_true, if_false),
            inspections: 0,
        })
    }
}

impl Monkey<Num> {
    fn round(monkeys: &mut [Self]) {
        for i in 0..monkeys.len() {
            let monkey = &monkeys[i];
            let dt1 = monkey.div_test.1;
            let dt2 = monkey.div_test.2;

            let (monkey, m1, m2) = match (dt1.cmp(&monkey.id), dt2.cmp(&monkey.id), dt1.cmp(&dt2)) {
                (Ordering::Less, Ordering::Greater, _) => {
                    let (r, rr) = monkeys.split_at_mut(dt2);
                    let (l, r) = r.split_at_mut(i);
                    (&mut r[0], &mut l[dt1], &mut rr[0])
                }
                (Ordering::Greater, Ordering::Less, _) => {
                    let (r, rr) = monkeys.split_at_mut(dt1);
                    let (l, r) = r.split_at_mut(i);
                    (&mut r[0], &mut rr[0], &mut l[dt2])
                }
                (Ordering::Greater, Ordering::Greater, Ordering::Greater) => {
                    let (r, rr) = monkeys.split_at_mut(dt1);
                    let (l, r) = r.split_at_mut(dt2);
                    (&mut l[i], &mut rr[0], &mut r[0])
                }
                (Ordering::Greater, Ordering::Greater, Ordering::Less) => {
                    let (r, rr) = monkeys.split_at_mut(dt2);
                    let (l, r) = r.split_at_mut(dt1);
                    (&mut l[i], &mut r[0], &mut rr[0])
                }
                (Ordering::Less, Ordering::Less, Ordering::Less) => {
                    let (r, rr) = monkeys.split_at_mut(i);
                    let (l, r) = r.split_at_mut(dt2);
                    (&mut rr[0], &mut l[dt1], &mut r[0])
                }
                (Ordering::Less, Ordering::Less, Ordering::Greater) => {
                    let (r, rr) = monkeys.split_at_mut(i);
                    let (l, r) = r.split_at_mut(dt1);
                    (&mut rr[0], &mut r[0], &mut l[dt2])
                }
                (o1 , o2 , o3 ) => {
                    eprintln!("Add this cmp branch: ({o1:?}, {o2:?}, {o3:?})");
                    panic!("Unexpected monkey division test ruling")
                }
            };

            monkey.inspections += monkey.items.len();

            while let Some(item) = monkey.items.pop_front() {
                //println!("Inspecting item with worry level {item}");
                let item = match monkey.operation {
                    Op::Sum(n) => item + n,
                    Op::Mul(n) => item * n,
                    Op::Square => item * item,
                };
                let item = item / 3;
                //println!("Item after operation {0:?}: {item}", monkey.operation);

                if item % monkey.div_test.0 as Num == 0 {
                    m1.items.push_back(item);
                } else {
                    m2.items.push_back(item);
                }
            }
        }
    }
}

pub fn task1(input: &str) -> SolutionResult {
    let mut monkeys: Vec<Monkey<Num>> = input.split("\n\n").map(|s| s.parse().unwrap()).collect();
    //println!("Start: {monkeys:#?}");

    for _i in 1..=20 {
        Monkey::round(&mut monkeys);
        //println!("After Round #{_i}:\n{monkeys:#?}");
    }

    let (first, second) =
        monkeys
            .iter()
            .map(|m| m.inspections)
            .fold((0, 0), |maxes, count| {
                match (count > maxes.0, count > maxes.1) {
                    (true, true) => (count, maxes.0),
                    (false, true) => (maxes.0, count),
                    (true, false) => panic!("Invalid finding max counts comparison"),
                    (false, false) => maxes,
                }
            });

    SolutionResult::Unsigned(first * second)
}

pub fn task2(input: &str) -> SolutionResult {
    SolutionResult::Unsigned(0)
}
