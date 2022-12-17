use std::{
    cmp::Ordering,
    collections::VecDeque,
    fmt::Debug,
    ops::{Add, Div, Mul},
    str::FromStr,
    string::ParseError,
    thread,
};

use crate::tooling::SolutionResult;

type Num = u64;

#[derive(Debug)]
enum WorryManagement {
    Div(Num),
    Mod(Num),
}

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
#[derive(Debug)]
struct Item {
    worry_lvl: Num,
    monkey: usize,
    worry_management: WorryManagement,
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

impl<T: Div + Mul + Add + Clone> Monkey<T> {
    fn clone_properties(&self) -> Monkey<T> {
        Monkey {
            id: self.id,
            items: Default::default(),
            operation: self.operation.clone(),
            div_test: self.div_test.clone(),
            inspections: Default::default(),
        }
    }
}

impl Monkey<Num> {
    fn round(monkeys: &mut [Self], worry_management: &WorryManagement) {
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

                let item = match worry_management {
                    WorryManagement::Div(d) => item / d,
                    WorryManagement::Mod(m) => item % m,
                };

                //println!("Item after operation {0:?}: {item}", monkey.operation);

                if item % monkey.div_test.0 as Num == 0 {
                    m1.items.push_back(item);
                } else {
                    m2.items.push_back(item);
                }
            }
        }
    }

    fn single_item_sim(mut item: Item, monkeys: &Vec<Self>, rounds: usize) -> Vec<usize> {
        let mut inspection_counts: Vec<usize> = Vec::with_capacity(monkeys.len());
        for _ in monkeys {
            inspection_counts.push(0)
        }

        for _ in 1..=rounds {
            for monkey in monkeys {
                if item.monkey != monkey.id {
                    continue;
                }

                item.worry_lvl = match monkey.operation {
                    Op::Sum(n) => item.worry_lvl + n,
                    Op::Mul(n) => item.worry_lvl * n,
                    Op::Square => item.worry_lvl * item.worry_lvl,
                };

                item.worry_lvl = match item.worry_management {
                    WorryManagement::Div(d) => item.worry_lvl / d,
                    WorryManagement::Mod(m) => item.worry_lvl % m,
                };

                //println!("Item after operation {0:?}: {item}", monkey.operation);

                inspection_counts[item.monkey] += 1;

                if item.worry_lvl % monkey.div_test.0 as Num == 0 {
                    item.monkey = monkey.div_test.1;
                } else {
                    item.monkey = monkey.div_test.2;
                }
            }
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
        Monkey::round(&mut monkeys, &WorryManagement::Div(3));
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
                worry_management: WorryManagement::Mod(worry_mod),
            })
        })
        .collect();

    //println!("MCM: {worry_mod}");

    let amount = monkeys.len();

    let inspection_counts: Vec<usize> = items
        .into_iter()
        .map(|item| {
            let mut vec = Vec::with_capacity(monkeys.len());
            for monkey in &monkeys {
                vec.push(monkey.clone_properties());
            }
            thread::spawn(move || Monkey::single_item_sim(item, &vec, 10000))
        })
        .collect::<Vec<_>>()
        .into_iter()
        .map(|handle| handle.join().unwrap())
        //.inspect(|i| println!("Inspections after rounds: {i:?}"))
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
