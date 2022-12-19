use colored::Colorize;
use std::{
    hint::black_box,
    time::{Duration, Instant},
};

use crate::tooling::{load_input, Solution};

pub fn benchmarks(inputs: &[[&'static str; 2]], solutions: &[[Solution; 2]], passes: u32) {
    if cfg!(debug_assertions) {
        eprintln!("{}: Benchmarking in debug build", "WARNING".yellow().bold());
    }

    let mut elapsed_total: Duration = Default::default();
    for (i, [f1, f2]) in solutions.iter().enumerate() {
        let day = i + 1;
        let data = || load_input(inputs, day, false);

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
