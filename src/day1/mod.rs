pub fn task1(lines: impl Iterator<Item = String>) -> i32 {
    // Tuple (current_sum, max_found)
    lines
        .fold((0, 0), |mut tuple, line| {
            if let Ok(i) = line.parse::<i32>() {
                tuple.0 += i;
            } else {
                tuple.0 = 0;
            }
            if tuple.0 > tuple.1 {
                tuple.1 = tuple.0
            };
            tuple
        })
        .1
}

pub fn task2(lines: impl Iterator<Item = String>) -> i32 {
    // Tuple (current_sum, elves_calories_vec)
    let mut sums: Vec<i32> = lines
        .fold((0, vec![0]), |mut tuple, line| {
            if let Ok(i) = line.parse::<i32>() {
                *(tuple.1.last_mut().unwrap()) += i;
            } else {
                tuple.0 = 0;
                tuple.1.push(0);
            }
            tuple
        })
        .1;

    sums.sort();
    sums.reverse();

    sums[0] + sums[1] + sums[2]
}
