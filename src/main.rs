#![feature(iter_array_chunks)]
#![feature(iter_next_chunk)]
#![feature(anonymous_lifetime_in_impl_trait)]
#![feature(bench_black_box)]

use clap::Parser;
use colored::Colorize;
use seq_macro::seq;
use std::{
    fmt::Debug,
    hint::black_box,
    time::{Duration, Instant},
};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

macro_rules! solutions {
    ($max_day:expr) => {
            seq!(N in 1..=$max_day {
            [
            #([
                |data: _| Box::new(day~N::task1(data)),
                |data: _| Box::new(day~N::task2(data))
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

// change max day here
const MAX_DAY: usize = 6;
const SOLUTIONS: [[fn(&str) -> Box<dyn Debug>; 2]; MAX_DAY] = solutions!(6); // and here!
const INPUTS: [[&str; 2]; MAX_DAY] = inputs!(6); // and here!

fn main() {
    let args = Args::parse();

    if args.bench {
        if cfg!(debug_assertions) {
            eprintln!(
                "{}: Benchmarking in debug build",
                format!("WARNING").yellow().bold()
            );
        }

        let mut elapsed_total: Duration = Default::default();
        for (i, [f1, f2]) in SOLUTIONS.iter().enumerate() {
            let day = i + 1;
            let data = || load_data(day, false);

            // Warm up cache
            for _ in 0..1000 {
                black_box(f1(data()));
            }

            let now = Instant::now();
            for _ in 0..5000 {
                black_box(f1(data()));
            }
            let elapsed1 = now.elapsed();
            let now = Instant::now();
            for _ in 0..5000 {
                black_box(f2(data()));
            }
            let elapsed2 = now.elapsed();
            println!(
                "\n{}: {}\n{}: {}",
                format!("day{day:02}/task1").bold(),
                format!("{:>10?}", elapsed1 / 5000).green(),
                format!("day{day:02}/task2").bold(),
                format!("{:>10?}", elapsed2 / 5000).green(),
            );

            elapsed_total += elapsed1 + elapsed2;
        }
        println!(
            "\n{}: {}",
            format!("Total").bold(),
            format!("{:>10?}", elapsed_total / 5000).green()
        );
    } else {
        let day = match args.day {
            Some(day) => day,
            None => {
                eprintln!("--day <DAY> argument not supplied");
                return;
            }
        };
        let data = || load_data(day, args.test);
        let (res1, res2): (Box<dyn Debug>, Box<dyn Debug>) = match day {
            day @ 1..=MAX_DAY => {
                let day = day - 1;
                (SOLUTIONS[day][0](data()), SOLUTIONS[day][1](data()))
            }
            26.. => {
                eprintln!("Day {} out of range (max 25)", day);
                return;
            }
            _ => {
                eprintln!("No solution available for day {}!", day);
                return;
            }
        };

        println!("Result 1: {:?}\nResult 2: {:?}", res1, res2);
    }
}

/// Will load a text file into lines which must be under `/src/dayXY/input.txt`
/// or `/src/dayXY/testInput.txt` depending on `load_test`
fn load_data(day: usize, load_test: bool) -> &'static str {
    return INPUTS[day - 1][load_test as usize];
}

/// Advent of Code 2022 solutions in Rust
#[derive(Parser, Debug)]
#[clap(author, about, long_about = None)]
struct Args {
    /// Load the testinput file in the dir
    #[clap(short = 't')]
    test: bool,

    /// Day's solutions to run
    #[clap(short, long)]
    day: Option<usize>,

    #[clap(long)]
    bench: bool,
}
