#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub patterns: Vec<String>,
    pub desired_designs: Vec<String>,
}

pub fn part1(input: &str) -> PuzzleInput {
    let (patterns, desired_designs) = input.split_once("\n\n").unwrap();
    let mut patterns: Vec<String> = patterns.split(", ").map(|s| s.to_string()).collect();
    let designs: Vec<String> = desired_designs
        .trim_end()
        .split("\n")
        .map(|s| s.to_string())
        .collect();

    patterns.sort_by_key(|p| p.len());
    patterns.reverse();

    PuzzleInput {
        patterns,
        desired_designs: designs,
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
