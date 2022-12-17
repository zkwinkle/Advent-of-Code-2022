#![feature(iter_array_chunks)]
#![feature(iter_next_chunk)]
#![feature(anonymous_lifetime_in_impl_trait)]
#![feature(associated_type_bounds)]

use clap::Parser;
use colored::Colorize;
use seq_macro::seq;
use std::{
    fmt::Debug,
    hint::black_box,
    time::{Duration, Instant},
};

#[macro_use]
mod tooling;
use tooling::*;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;

// change max day here
const MAX_DAY: usize = 11;
const SOLUTIONS: [[Solution; 2]; MAX_DAY] = solutions!(11); // and here!
const INPUTS: [[&str; 2]; MAX_DAY] = inputs!(11); // and here!

fn main() {
    let args = Args::parse();

    if let Some(passes_opt) = args.bench {
        let passes = passes_opt.unwrap_or(50);
        benchmarks(passes)
    } else {
        let day = match args.day {
            Some(day) => day,
            None => {
                eprintln!("--day <DAY> argument not supplied");
                return;
            }
        };
        let data = || load_data(day, args.test);
        let (res1, res2): (SolutionResult, SolutionResult) = match day {
            day @ 1..=MAX_DAY => {
                let day = day - 1;
                (SOLUTIONS[day][0](data()), SOLUTIONS[day][1](data()))
            }
            26.. => {
                eprintln!("Day {day} out of range (max 25)");
                return;
            }
            _ => {
                eprintln!("No solution available for day {day}!");
                return;
            }
        };

        println!("Result 1: {res1}\nResult 2: {res2}");
    }
}

fn benchmarks(passes: u32) {
    if cfg!(debug_assertions) {
        eprintln!("{}: Benchmarking in debug build", "WARNING".yellow().bold());
    }

    let mut elapsed_total: Duration = Default::default();
    for (i, [f1, f2]) in SOLUTIONS.iter().enumerate() {
        let day = i + 1;
        let data = || load_data(day, false);

        let now = Instant::now();
        for _ in 0..passes {
            black_box(f1(data()));
        }
        let elapsed1 = now.elapsed();
        let now = Instant::now();
        for _ in 0..passes {
            black_box(f2(data()));
        }
        let elapsed2 = now.elapsed();
        println!(
            "\n{}: {}\n{}: {}",
            format!("day{day:02}/task1").bold(),
            format!("{:>10?}", elapsed1 / passes).green(),
            format!("day{day:02}/task2").bold(),
            format!("{:>10?}", elapsed2 / passes).green(),
        );

        elapsed_total += elapsed1 + elapsed2;
    }
    println!(
        "\n{}: {}",
        "Total".bold(),
        format!("{:>10?}", elapsed_total / passes).green()
    );
}

fn load_data(day: usize, load_test: bool) -> &'static str {
    INPUTS[day - 1][load_test as usize]
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
    bench: Option<Option<u32>>,
}
