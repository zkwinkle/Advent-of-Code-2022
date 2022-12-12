use crate::tooling::SolutionResult;

#[derive(Clone, Copy)]
enum Tool {
    Rock,
    Paper,
    Scissors,
}

impl Tool {
    fn from_char(c: char) -> Tool {
        match c {
            'A' | 'X' => Tool::Rock,
            'B' | 'Y' => Tool::Paper,
            'C' | 'Z' => Tool::Scissors,
            _ => panic!("Invalid char passed to Tool::from_char(): '{c}'"),
        }
    }

    fn from_chars_task2(chars: (char, char)) -> (Tool, Tool) {
        let other_tool = Tool::from_char(chars.0);
        (
            other_tool,
            match chars.1 {
                'X' => match other_tool {
                    // lose
                    Tool::Rock => Tool::Scissors,
                    Tool::Scissors => Tool::Paper,
                    Tool::Paper => Tool::Rock,
                },
                'Y' => Tool::from_char(chars.0), //draw
                'Z' => match other_tool {
                    // win
                    Tool::Rock => Tool::Paper,
                    Tool::Scissors => Tool::Rock,
                    Tool::Paper => Tool::Scissors,
                },
                _ => panic!(
                    "Invalid char passed to Tool::from_chars_task2(): '{}'",
                    chars.1
                ),
            },
        )
    }

    fn value(&self) -> i32 {
        match self {
            Tool::Rock => 1,
            Tool::Paper => 2,
            Tool::Scissors => 3,
        }
    }

    fn fight(&self, other: Tool) -> i32 {
        self.value()
            + match (self, other) {
                (Tool::Rock, Tool::Paper)
                | (Tool::Paper, Tool::Scissors)
                | (Tool::Scissors, Tool::Rock) => 0, //lose

                (Tool::Rock, Tool::Rock)
                | (Tool::Scissors, Tool::Scissors)
                | (Tool::Paper, Tool::Paper) => 3, // draw

                (Tool::Rock, Tool::Scissors)
                | (Tool::Scissors, Tool::Paper)
                | (Tool::Paper, Tool::Rock) => 6, // win
            }
    }
}

// A / X: rock = 1pt
// B / Y: paper = 2pt
// C / Z: scissors = 3 pt
// lose = 0pt
// draw = 3pt
// win  = 6pt
pub fn task1(input: &str) -> SolutionResult {
    // Tuple (current_sum, max_found)
    SolutionResult::Signed(input.lines().fold(0, |score, line| {
        let mut letters = line.trim().chars();
        let other = Tool::from_char(letters.next().unwrap());
        let my = Tool::from_char(letters.last().unwrap());
        score + my.fight(other)
    }))
}

pub fn task2(input: &str) -> SolutionResult {
    // Tuple (current_sum, elves_calories_vec)
    SolutionResult::Signed(input.lines().fold(0, |score, line| {
        let mut letters = line.trim().chars();
        let (other, my) =
            Tool::from_chars_task2((letters.next().unwrap(), letters.last().unwrap()));
        score + my.fight(other)
    }))
}
