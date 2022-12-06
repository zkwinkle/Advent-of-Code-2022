#![feature(iter_array_chunks)]
#![feature(iter_next_chunk)]

use clap::Parser;
use seq_macro::seq;
use std::{
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader},
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

const SOLUTIONS: [[fn(Box<dyn Iterator<Item = String>>) -> Box<dyn Debug>; 2]; 6] = solutions!(6);

fn main() {
    let args = Args::parse();

    let data = || load_data(args.day, args.test);
    let (res1, res2): (Box<dyn Debug>, Box<dyn Debug>) = match args.day {
        day @ 1..=6 => {
            let day = day - 1;
            (SOLUTIONS[day][0](data()), SOLUTIONS[day][1](data()))
        }
        26.. => {
            eprintln!("Day {} out of range (max 25)", args.day);
            return;
        }
        _ => {
            eprintln!("No solution available for day {}!", args.day);
            return;
        }
    };

    println!("Result 1: {:?}\nResult 2: {:?}", res1, res2);
}

/// Will load a text file into lines which must be under `/src/dayXY/input.txt`
/// or `/src/dayXY/testInput.txt` depending on `load_test`
fn load_data(day: usize, load_test: bool) -> Box<dyn Iterator<Item = String>> {
    let file_name = if load_test { "testinput" } else { "input" };
    let file_name = format!("./src/day{}/{}.txt", day, file_name);
    let f =
        File::open(&file_name).unwrap_or_else(|_| panic!("Couldn't open input file {}", file_name));
    let reader = BufReader::new(f);

    Box::new(
        reader
            .lines()
            .map(|line_res| line_res.expect("IO Error reading input file")),
    )
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
    day: usize,
}
