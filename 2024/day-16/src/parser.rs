use utility_belt::prelude::*;

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    // Remember to make the fields pub
    pub maze: Grid2D<char>,
}

pub fn part1(input: &str) -> PuzzleInput {
    PuzzleInput { maze: input.into() }
}

pub fn part2(input: &str) -> PuzzleInput {
    part1(input)
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = indoc! {"
        TODO
    "};

    #[test]
    fn test_parse() {
        assert_ne!(TEST_INPUT, "TODO");
        assert!(trace("Puzzle", winnow_parser).parse_next(&mut TEST_INPUT.clone()).is_ok());
    }
}
*/
