use std::{ops::Range, str::FromStr, string::ParseError};

use aoc_lib::{
    parsing::get_numbers, structs::position::Position, tooling::SolutionResult,
};

type Num = i32;

#[derive(Debug)]
struct Sensor {
    position: Position<Num>,
    beacon: Position<Num>,
    // Radius is measured with manhattan distance
    radius: usize,
}

impl Sensor {
    pub fn new(sensor: Position<Num>, beacon: Position<Num>) -> Sensor {
        let distance = sensor - beacon;

        let radius: usize = TryInto::<usize>::try_into(distance.x.abs())
            .unwrap()
            + TryInto::<usize>::try_into(distance.y.abs()).unwrap();

        Sensor {
            position: sensor,
            beacon,
            radius,
        }
    }
}

impl FromStr for Sensor {
    type Err = ParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let nums: Vec<Num> = get_numbers(line).unwrap();

        let sensor: Position<Num> = Position::new(nums[0], nums[1]);

        let beacon: Position<Num> = Position::new(nums[2], nums[3]);

        Ok(Sensor::new(sensor, beacon))
    }
}

impl Sensor {
    fn row_coverage(&self, row: Num) -> Range<Num> {
        let y_diff = row.abs_diff(self.position.y) as usize;
        if y_diff > self.radius {
            return 0..0;
        }

        let offset = (self.radius - y_diff) as Num;

        return (self.position.x - offset)..(self.position.x + offset + 1);
    }
}

fn parse(input: &str) -> Vec<Sensor> {
    get_numbers(input)
        .unwrap()
        .chunks(4)
        .map(|nums| {
            let sensor: Position<Num> = Position::new(nums[0], nums[1]);

            let beacon: Position<Num> = Position::new(nums[2], nums[3]);
            Sensor::new(sensor, beacon)
        })
        .collect()
}

fn count_row(sensors: Vec<Sensor>, row: Num) -> usize {
    let mut ranges: Vec<Range<_>> =
        sensors.iter().map(|s| s.row_coverage(row)).collect();

    for i in 0..ranges.len() {
        let (left, right) = ranges.split_at_mut(i + 1);
        let r1 = left.last_mut().unwrap();

        for r2 in right.iter() {
            match (r2.contains(&r1.start), r2.contains(&r1.end)) {
                (true, true) => {
                    *r1 = 0..0;
                }
                (true, false) => {
                    r1.start = r2.end;
                }
                (false, true) => {
                    r1.end = r2.start;
                }
                (false, false) => (),
            }
        }
    }

    (ranges.iter().fold(0, |count, r| count + (r.end - r.start)) - 1)
        .try_into()
        .unwrap()

    //let beacons = sensors.iter().filter_map(|s| {
    //    if s.beacon.y != row {
    //        None
    //    } else {
    //        Some(s.beacon.x)
    //    }
    //});

    //for beacon in beacons {
    //    if ranges.iter().any(|r| r.contains(&beacon)) {
    //        count -= 1;
    //    }
    //}
}

pub fn task1(input: &str) -> SolutionResult {
    let sensors: Vec<Sensor> = parse(input);

    // TODO: Make a function that gets the ranges covered by each beacon for a
    // specific row. Then count the total tiles covered by checking overlaps
    // and stuff and discount the beacons that are within those ranges.

    let y = 2000000;

    SolutionResult::Unsigned(count_row(sensors, y))
}

pub fn task2(input: &str) -> SolutionResult { SolutionResult::Unsigned(0) }
