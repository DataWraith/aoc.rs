use crate::{p1::polymer_reaction, parser::*};

pub fn part2(input: &PuzzleInput) -> String {
    polymer_reaction(input, 40)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;

    #[test]
    fn test_part2_example() {
        let input = parser::part2(parser::TEST_INPUT);
        assert_ne!(parser::TEST_INPUT.trim(), "TODO");
        assert_eq!(part2(&input), "2188189693529");
    }
}
