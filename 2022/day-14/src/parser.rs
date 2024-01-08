use utility_belt::prelude::*;

use crate::structs::*;

fn nom_parser(input: &str) -> IResult<&str, PuzzleInput> {
    let (input, segments) = many1(parse_segment)(input)?;
    let (input, _) = eof(input)?;

    Ok((input, PuzzleInput { segments }))
}

pub fn parse_segment(input: &str) -> IResult<&str, Vec<Coordinate>> {
    let (input, segments) = separated_list1(tag(" -> "), parse_coordinate)(input)?;
    let (input, _) = newline(input)?;

    Ok((input, segments))
}

pub fn parse_coordinate(input: &str) -> IResult<&str, Coordinate> {
    let (input, x) = i32(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, y) = i32(input)?;

    Ok((input, Coordinate::new(x, y)))
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
