use utility_belt::prelude::*;

use crate::parser::*;

pub fn part1(input: &PuzzleInput) -> String {
    let mut coord = Coordinate::new(0, 0);

    for command in input.commands.iter() {
        match command {
            Command::Forward(amount) => coord.x += amount,
            Command::Up(amount) => coord.y -= amount,
            Command::Down(amount) => coord.y += amount,
        }
    }

    format!("{}", coord.x * coord.y)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = indoc! {"
        forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2
    "};

    #[test]
    fn test_part1_example() {
        let input = crate::parser::part1(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(part1(&input), "150");
    }
}
