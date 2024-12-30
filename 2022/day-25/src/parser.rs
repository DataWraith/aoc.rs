#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub numbers: Vec<i64>,
}

pub fn part1(input: &'static str) -> PuzzleInput {
    let mut numbers = Vec::new();

    for line in input.lines() {
        let number = parse_snafu(line.trim());
        numbers.push(number);
    }

    dbg!(&numbers.len());

    PuzzleInput { numbers }
}

fn parse_snafu(s: &str) -> i64 {
    let mut result = 0;
    let mut coef = 1;

    for c in s.chars().rev() {
        let digit = match c {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!("Invalid character: {}", c),
        };

        result += digit * coef;
        coef *= 5;
    }

    result
}
