use utility_belt::prelude::*;

use crate::parser::*;

pub fn part2(input: &PuzzleInput) -> String {
    todo!("day_11::p2::part2");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;
    use utility_belt::prelude::*;

    #[test]
    fn test_part2_example() {
        let input = parser::part2(parser::TEST_INPUT);
        assert_ne!(parser::TEST_INPUT.trim(), "TODO");
        assert_eq!(part2(&input), "TODO");
    }
}
