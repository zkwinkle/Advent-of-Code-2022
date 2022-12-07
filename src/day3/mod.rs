use crate::tooling::SolutionResult;

fn find_repeat(iter1: &str, iter2: &str) -> char {
    iter1.chars().find(|&c1| iter2.contains(c1)).unwrap()
}

fn get_priority(c: char) -> i32 {
    match c {
        'a'..='z' => (c as i32) - 96,
        'A'..='Z' => (c as i32) - 38,
        _ => panic!("Unexpected char passed to get_priority: '{}'", c),
    }
}

pub fn task1(input: &str) -> SolutionResult {
    SolutionResult::Signed(input.lines().fold(0, |acc, line| {
        let length = line.chars().count();
        let repeat_char = find_repeat(&line[..length / 2], &line[length / 2..]);
        //println!("Repeat char found in line {}: '{}'\tValue:{}",line,repeat_char, get_priority(repeat_char));
        acc + get_priority(repeat_char)
    }))
}

fn find_badge(lines: &[&str; 3]) -> char {
    lines[0]
        .chars()
        .find(|&c1| lines[1].contains(c1) && lines[2].contains(c1))
        .unwrap()
}

pub fn task2(input: &str) -> SolutionResult {
    SolutionResult::Signed(input.lines().array_chunks::<3>().fold(0, |acc, ref lines| {
        let repeat_char = find_badge(lines);
        //println!( "Repeat char found in lines {:?}: '{}'\tValue:{}", &lines, repeat_char, get_priority(repeat_char));
        acc + get_priority(repeat_char)
    }))
}
