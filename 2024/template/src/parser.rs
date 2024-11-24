use winnow::{PResult, Parser, combinator::trace};

use crate::structs::*;

fn winnow_parser<'s>(input: &mut &'s str) -> PResult<PuzzleInput> {
    Ok(PuzzleInput{})
}

pub fn parse(input: &str) -> PuzzleInput {
    winnow_parser.parse(input).unwrap()
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
