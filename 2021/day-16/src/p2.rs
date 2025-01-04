use utility_belt::prelude::*;

use crate::parser::*;

pub fn part2(input: &PuzzleInput) -> String {
    todo!("day_16::p2::part2");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;
    use utility_belt::prelude::*;

    #[test]
    fn test_part2_example() {
        let input = parser::part2(parser::TEST_INPUT1);
        assert_ne!(parser::TEST_INPUT1.trim(), "TODO");
        assert_eq!(part2(&input), "TODO");
    }
}
