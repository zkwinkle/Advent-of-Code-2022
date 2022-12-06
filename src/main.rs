#![feature(iter_array_chunks)]
#![feature(iter_next_chunk)]

use std::{
    fs::File,
    io::{BufRead, BufReader},
};
use std::fmt::Debug;

use clap::Parser;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() {
    let args = Args::parse();

    let data = || load_data(args.day, args.test);

    let (res1, res2): (Box<dyn Debug>, Box<dyn Debug>) = match args.day {
        1 => (Box::new(day1::task1(data())), Box::new(day1::task2(data()))),
        2 => (Box::new(day2::task1(data())), Box::new(day2::task2(data()))),
        3 => (Box::new(day3::task1(data())), Box::new(day3::task2(data()))),
        4 => (Box::new(day4::task1(data())), Box::new(day4::task2(data()))),
        5 => (Box::new(day5::task1(data())), Box::new(day5::task2(data()))),
        6 => (Box::new(day6::task1(data())), Box::new(day6::task2(data()))),
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
fn load_data(day: usize, load_test: bool) -> impl Iterator<Item = String> {
    let file_name = if load_test { "testinput" } else { "input" };
    let file_name = format!("./src/day{}/{}.txt", day, file_name);
    let f =
        File::open(&file_name).unwrap_or_else(|_| panic!("Couldn't open input file {}", file_name));
    let reader = BufReader::new(f);

    reader
        .lines()
        .map(|line_res| line_res.expect("IO Error reading input file"))
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
