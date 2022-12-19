use aoc_lib::tooling::SolutionResult;

pub fn task1(input: &str) -> SolutionResult {
    let mut lines = input.lines();
    let line: Vec<char> = lines.next().unwrap().chars().collect();

    SolutionResult::Unsigned(
        line.windows(4)
            .position(|slice| {
                !slice
                    .iter()
                    .enumerate()
                    .any(|(i, c1)| slice[i + 1..].contains(c1))
            })
            .unwrap()
            + 4,
    )
}

pub fn task2(input: &str) -> SolutionResult {
    let mut lines = input.lines();
    let line: Vec<char> = lines.next().unwrap().chars().collect();

    SolutionResult::Unsigned(
        line.windows(14)
            .position(|slice| {
                !slice
                    .iter()
                    .enumerate()
                    .any(|(i, c1)| slice[i + 1..].contains(c1))
            })
            .unwrap()
            + 14,
    )
}
