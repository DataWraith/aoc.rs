use utility_belt::prelude::*;

use crate::parser::*;

pub fn part2(input: &PuzzleInput) -> String {
    todo!("{{crate_name}}::p2::part2");
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = indoc! {"
        TODO
    "};

    #[test]
    fn test_part2_example() {
        let input = crate::parser::part2(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(part2(&input), "TODO");
    }
}
