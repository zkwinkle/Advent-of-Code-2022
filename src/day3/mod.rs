pub fn find_repeat(mut iter1: impl Iterator<Item=char>, iter2: impl Clone + Iterator<Item=char>) -> char {
    iter1.find(|&c1| {
        iter2.clone().find(|&c2| {
            c1 == c2
        }).is_some()
    }).unwrap()
}

pub fn get_priority(c: char) -> i32{
    if c.is_ascii_lowercase() {
        return (c as i32) - 96
    }
    else if c.is_ascii_uppercase() {
        return (c as i32) - 38
    }
    else {
        panic!("Unexpected char passed to get_priority: '{}'", c);
    }
}

pub fn task1(lines: impl Iterator<Item = String>) -> i32 {
    lines.fold(0, |acc, line| {
        let length = line.chars().count();
        let repeat_char = find_repeat(line.chars().take(length/2), line.chars().skip(length/2));
        //println!("Repeat char found in line {}: '{}'\tValue:{}",line,repeat_char, get_priority(repeat_char));
        acc + get_priority(repeat_char)
    })
}

pub fn task2(lines: impl Iterator<Item = String>) -> i32 {
    0
}
