#![feature(iter_array_chunks)]
#![feature(iter_next_chunk)]
#![feature(anonymous_lifetime_in_impl_trait)]
#![feature(associated_type_bounds)]

use clap::Parser;
use std::fmt::Debug;

use aoc_lib::{benchmark::benchmarks, inputs, solutions, tooling::*};

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

// change max day here
const MAX_DAY: usize = 13;
const SOLUTIONS: [[Solution; 2]; MAX_DAY] = solutions!(13); // and here!
const INPUTS: [[&str; 2]; MAX_DAY] = inputs!(13); // and here!

fn main() {
    let args = Args::parse();

    if let Some(passes_opt) = args.bench {
        let passes = passes_opt.unwrap_or(100);
        benchmarks(&INPUTS, &SOLUTIONS, passes)
    } else {
        let day = match args.day {
            Some(day) => day,
            None => {
                eprintln!("--day <DAY> argument not supplied");
                return;
            }
        };
        let data = || load_input(&INPUTS, day, args.test);
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
