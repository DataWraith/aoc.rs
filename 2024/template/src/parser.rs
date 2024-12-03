use winnow::{PResult, Parser, combinator::trace};

use crate::structs::*;

fn winnow_parser<'s>(input: &mut &'s str) -> PResult<PuzzleInput> {
    Ok(PuzzleInput{})
}

pub fn part1(input: &str) -> PuzzleInput {
    // Checklist:
    //
    // 1. Can this be parsed using parse_ints(input)
    // 2. Can this be parsed using a regular expression
    // 3. Winnow parser?
    // 4. ???

    winnow_parser.parse(input).unwrap()
}

pub fn part2(input: &str) -> PuzzleInput {
    part1(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = indoc! {"
        TODO
    "};

    #[test]
    fn test_parse() {
        assert!(trace("Puzzle", winnow_parser).parse_next(&mut TEST_INPUT.clone()).is_ok());
    }
}
