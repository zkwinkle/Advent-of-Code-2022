use std::{num::ParseIntError, str::FromStr};

#[derive(Debug)]
struct Range {
    min: u32,
    max: u32,
}

impl Range {
    fn contains(&self, other: &Self) -> bool {
        self.min <= other.min && self.max >= other.max
    }

    fn overlaps(&self, other: &Self) -> bool {
        (self.min >= other.min && self.min <= other.max)
            || (self.max >= other.min && self.max <= other.max)
            || self.contains(other)
    }
}

impl FromStr for Range {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (min, max) = s.split_once('-').unwrap();

        let min_fromstr = min.parse::<u32>()?;
        let max_fromstr = max.parse::<u32>()?;

        Ok(Range {
            min: min_fromstr,
            max: max_fromstr,
        })
    }
}

pub fn task1(input: &str) -> i32 {
    input.lines().fold(0, |acc, line| {
        let ranges: (&str, &str) = line.split_once(',').unwrap();
        let ranges: (Range, Range) = (ranges.0.parse().unwrap(), ranges.1.parse().unwrap());

        acc + if ranges.0.contains(&ranges.1) || ranges.1.contains(&ranges.0) {
            1
        } else {
            0
        }
    })
}

pub fn task2(input: &str) -> i32 {
    input.lines().fold(0, |acc, line| {
        let ranges: (&str, &str) = line.split_once(',').unwrap();
        let ranges: (Range, Range) = (ranges.0.parse().unwrap(), ranges.1.parse().unwrap());

        //println!(
        //    "Ranges: {:?}\toverlap? {}",
        //    ranges,
        //    ranges.0.overlaps(&ranges.1)
        //);

        acc + if ranges.0.overlaps(&ranges.1) { 1 } else { 0 }
    })
}
