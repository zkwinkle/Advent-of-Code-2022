use crate::tooling::SolutionResult;

pub fn task1(input: &str) -> SolutionResult {
    let mut lines = input.lines();
    let line: Vec<char> = lines.next().unwrap().chars().collect();

    SolutionResult::Unsigned(
        line.windows(4)
            .position(|slice| {
                slice
                    .iter().enumerate()
                    .find(|(i, c1)| slice[i+1..].contains(c1))
                    .is_none()
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
                slice
                    .iter().enumerate()
                    .find(|(i, c1)| slice[i+1..].contains(c1))
                    .is_none()
            })
            .unwrap()
            + 14,
    )
}
