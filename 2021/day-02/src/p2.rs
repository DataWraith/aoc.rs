use crate::parser::*;

pub struct State {
    pub x: i32,
    pub depth: i32,
    pub aim: i32,
}

pub fn part2(input: &PuzzleInput) -> String {
    let mut state = State {
        x: 0,
        depth: 0,
        aim: 0,
    };

    for command in input.commands.iter() {
        match command {
            Command::Forward(amount) => {
                state.x += amount;
                state.depth += state.aim * amount;
            }
            Command::Up(amount) => state.aim -= amount,
            Command::Down(amount) => state.aim += amount,
        }
    }

    format!("{}", state.x * state.depth)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = utility_belt::prelude::indoc! {"
        forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2
    "};

    #[test]
    fn test_part2_example() {
        let input = crate::parser::part2(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(part2(&input), "900");
    }
}
