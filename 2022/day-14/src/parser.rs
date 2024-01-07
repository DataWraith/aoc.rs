use utility_belt::prelude::*;

use crate::structs::*;

fn nom_parser(input: &str) -> IResult<&str, PuzzleInput> {
    todo!();
    let (input, _) = eof(input)?;

    Ok((input, PuzzleInput {}))
}

pub fn parse(input: &str) -> PuzzleInput {
    nom_parser(input).unwrap().1
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_parse() {
        assert!(nom_parser(TEST_INPUT).is_ok());
    }
}
