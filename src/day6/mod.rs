use crate::tooling::SolutionResult;

pub fn task1(input: &str) -> SolutionResult {
    let mut lines = input.lines();
    let line: Vec<char> = lines.next().unwrap().chars().collect();

    SolutionResult::Unsigned(
        line.windows(4)
            .position(|slice| {
                slice
                    .iter()
                    .find(|&&c1| slice.iter().filter(|&&c2| c1 != c2).count() <= 2)
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
                    .iter()
                    .find(|&&c1| slice.iter().filter(|&&c2| c1 != c2).count() <= 12)
                    .is_none()
            })
            .unwrap()
            + 14,
    )
}
