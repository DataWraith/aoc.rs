use utility_belt::prelude::*;

use crate::parser::*;

pub fn part1(input: &PuzzleInput) -> String {
    dbg!(&input);
    todo!("{{crate_name}}::p1::part1");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;
    use utility_belt::prelude::*;

    #[test]
    fn test_part1_example() {
        let input = parser::part1(parser::TEST_INPUT);
        assert_ne!(parser::TEST_INPUT.trim(), "TODO");
        assert_eq!(part1(&input), "TODO");
    }
}
