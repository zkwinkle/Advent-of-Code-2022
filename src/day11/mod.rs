use std::{
    cmp::Ordering,
    collections::VecDeque,
    fmt::Debug,
    ops::{Add, Div, Mul, Range},
    str::FromStr,
    string::ParseError,
};

use aoc_lib::tooling::SolutionResult;

type Num = u64;

#[derive(Debug, Clone)]
enum Op<M: Mul + Clone, A: Add + Clone> {
    Sum(A),
    Mul(M),
    Square,
}

#[derive(Debug)]
struct Monkey<T: Div + Mul + Add + Clone> {
    id: usize,
    items: VecDeque<T>,
    operation: Op<T, T>,

    /// Tuple: (factor, who to pass if divisible, who to pass if not divisible)
    div_test: (T, usize, usize),

    inspections: usize,
}

// For simulating a single item at a time (optimization)
#[derive(Clone, Copy, Debug, PartialEq)]
struct Item {
    worry_lvl: Num,
    monkey: usize,
}

impl<T> FromStr for Monkey<T>
where
    T: Div + Mul + Add + FromStr<Err: Debug> + Clone,
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
    /// Used for task1
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
                (o1, o2, o3) => {
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

    /// Used for task2
    fn single_item_sim(item: Item, monkeys: &Vec<Self>, rounds: usize, modulo: Num) -> Vec<usize> {
        let mut inspection_counts: Vec<usize> = vec![0; monkeys.len()];

        let mut inspected: Vec<bool> = vec![false; monkeys.len()];

        let f = |mut item: Item, mut inspected: Option<&mut Vec<bool>>| {
            if let Some(ref mut inspected) = inspected {
                inspected.fill(false);
            }
            for monkey in monkeys {
                if item.monkey != monkey.id {
                    continue;
                }

                item.worry_lvl = match monkey.operation {
                    Op::Sum(n) => item.worry_lvl + n,
                    Op::Mul(n) => item.worry_lvl * n,
                    Op::Square => item.worry_lvl * item.worry_lvl,
                };

                item.worry_lvl %= modulo;

                //println!("Item after operation {0:?}: {item}", monkey.operation);
                if let Some(ref mut inspected) = inspected {
                    inspected[item.monkey] = true;
                }

                if item.worry_lvl % monkey.div_test.0 as Num == 0 {
                    item.monkey = monkey.div_test.1;
                } else {
                    item.monkey = monkey.div_test.2;
                }
            }
            item
        };

        // Brent's cycle detecting algorithm
        let mut power = 1;
        let mut lam = 1;
        let mut tortoise = item;
        let mut hare = f(item, None);

        while tortoise != hare {
            if power == lam {
                tortoise = hare;
                power *= 2;
                lam = 0
            }
            let next = f(hare, None);
            hare = next;
            lam += 1;
        }

        tortoise = item;
        hare = item;
        for _ in 0..lam {
            hare = f(hare, None);
        }

        let mut mu = 0;
        while tortoise != hare {
            tortoise = f(tortoise, None);
            hare = f(hare, None);
            mu += 1;
        }

        let mut item = item;
        let mut count_inspections_range = |range: Range<usize>| {
            range.fold(inspection_counts.clone(), |mut vec, _| {
                item = f(item, Some(&mut inspected));
                for (i, &inspection) in inspected.iter().enumerate() {
                    if inspection {
                        vec[i] += 1;
                    }
                }
                vec
            })
        };
        let mu_inspections = count_inspections_range(0..mu);
        let cycle_inspections = count_inspections_range(0..lam);
        let final_inspections = count_inspections_range(0..((rounds - mu) % lam));

        for i in 0..inspection_counts.len() {
            inspection_counts[i] = cycle_inspections[i] * ((rounds - mu) / lam)
                + mu_inspections[i]
                + final_inspections[i];
        }

        inspection_counts
    }
}

fn max2<T: PartialOrd + Default>(iter: impl Iterator<Item = T>) -> (T, T) {
    iter.fold(
        (Default::default(), Default::default()),
        |maxes, count| match (count > maxes.0, count > maxes.1) {
            (true, true) => (count, maxes.0),
            (false, true) => (maxes.0, count),
            (true, false) => panic!("Invalid finding max counts comparison"),
            (false, false) => maxes,
        },
    )
}

pub fn task1(input: &str) -> SolutionResult {
    let mut monkeys: Vec<Monkey<Num>> = input.split("\n\n").map(|s| s.parse().unwrap()).collect();

    for _i in 1..=20 {
        Monkey::round(&mut monkeys);
        //println!("After Round #{_i}:\n{monkeys:#?}");
    }

    //for monkey in &monkeys {
    //    println!("Inspections of Monkey{}: {}", monkey.id, monkey.inspections);
    //}

    let (first, second) = max2(monkeys.iter().map(|m| m.inspections));

    SolutionResult::Unsigned(first * second)
}

pub fn task2(input: &str) -> SolutionResult {
    let monkeys: Vec<Monkey<Num>> = input.split("\n\n").map(|s| s.parse().unwrap()).collect();

    let worry_mod = monkeys.iter().fold(1, |mcm, m| mcm * m.div_test.0);

    let items: Vec<Item> = monkeys
        .iter()
        .flat_map(|monkey| {
            monkey.items.iter().map(|&item| Item {
                worry_lvl: item,
                monkey: monkey.id,
            })
        })
        .collect();

    //println!("MCM: {worry_mod}");

    let amount = monkeys.len();

    let inspection_counts: Vec<usize> = items
        .into_iter()
        .map(|item| Monkey::single_item_sim(item, &monkeys, 10000, worry_mod))
        .fold(
            [0].repeat(amount),
            |mut counts: Vec<usize>, item_inspections: Vec<usize>| {
                for (i, inspections) in item_inspections.iter().enumerate() {
                    counts[i] += inspections;
                }
                counts
            },
        );

    let (first, second) = max2(inspection_counts.into_iter());

    SolutionResult::Unsigned(first * second)
}
