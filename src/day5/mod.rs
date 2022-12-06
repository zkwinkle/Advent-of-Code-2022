fn init_crates(lines: &mut impl Iterator<Item = &str>) -> Vec<Vec<char>> {
    let mut lines_peek = lines.peekable();
    let length = (lines_peek.peek().unwrap().chars().count() + 1) / 4;
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); length];

    for line in lines_peek {
        if !line.trim().starts_with('[') {
            break;
        }

        let mut block_iter = line.chars();
        for stack in &mut stacks {
            let block: char = match block_iter.next_chunk::<4>() {
                Ok(val) => val[1],
                Err(mut val) => val.nth(1).unwrap(),
            };
            if !block.is_whitespace() {
                stack.insert(0, block);
            }
        }
    }

    // Advance lines past blank line
    lines.next();

    stacks
}

pub fn task1(input: &str) -> String {
    let mut lines = input.lines();
    let mut stacks: Vec<Vec<char>> = init_crates(&mut lines);

    for line in lines {
        let instructions: Vec<usize> = line
            .split_ascii_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        let ammt = instructions[0];
        let from = instructions[1] - 1;
        let to = instructions[2] - 1;
        for _ in 0..ammt {
            let c = stacks[from].pop().unwrap();
            stacks[to].push(c);
        }
    }

    stacks
        .into_iter()
        .map(|stack| *stack.last().unwrap())
        .collect()
}

pub fn task2(input: &str) -> String {
    let mut lines = input.lines();
    let mut stacks: Vec<Vec<char>> = init_crates(&mut lines);

    for line in lines {
        let instructions: Vec<usize> = line
            .split_ascii_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        let ammt = instructions[0];

        // Getting the specific stacks using split_at_mut is a bit clunky
        let (from, to) = if instructions[1] < instructions[2] {
            let (first_half, second_half) = stacks.split_at_mut(instructions[2] - 1);
            (
                first_half.get_mut(instructions[1] - 1).unwrap(),
                second_half.get_mut(0).unwrap(),
            )
        } else {
            let (first_half, second_half) = stacks.split_at_mut(instructions[1] - 1);
            (
                second_half.get_mut(0).unwrap(),
                first_half.get_mut(instructions[2] - 1).unwrap(),
            )
        };

        let moved_blocks = from.drain(from.len() - ammt..from.len());
        to.extend(moved_blocks);
    }

    stacks
        .into_iter()
        .map(|stack| *stack.last().unwrap())
        .collect()
}
