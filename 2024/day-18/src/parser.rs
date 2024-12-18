use utility_belt::prelude::*;

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub bytes: Vec<Coordinate>,
    pub width: usize,
    pub height: usize,
    pub to_simulate: usize,
}

pub fn part1(input: &str) -> PuzzleInput {
    let bytes = parse_uints(input)
        .chunks(2)
        .map(|w| Coordinate::new(w[0] as i32, w[1] as i32))
        .collect();

    let width = 71;
    let height = 71;
    let to_simulate = 1024;

    PuzzleInput {
        bytes,
        width,
        height,
        to_simulate,
    }
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
