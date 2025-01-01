use crate::{p1::step, parser::*};

pub fn part2(input: &PuzzleInput) -> String {
    let mut octopi = input.octopi.clone();

    for step_idx in 1.. {
        if step(&mut octopi) == octopi.width() * octopi.height() {
            return step_idx.to_string();
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;

    #[test]
    fn test_part2_example() {
        let input = parser::part2(parser::TEST_INPUT);
        assert_ne!(parser::TEST_INPUT.trim(), "TODO");
        assert_eq!(part2(&input), "195");
    }
}
