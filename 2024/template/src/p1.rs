use crate::structs::*;

use utility_belt::prelude::*;

#[tracing::instrument(skip(input))]
pub fn part1(input: &PuzzleInput) -> String {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = indoc! {"
        TODO
    "};

    #[test]
    fn test_part1() {
        let input = crate::parser::part1(TEST_INPUT);
        assert_eq!(part1(&input), "TODO");
    }
}
