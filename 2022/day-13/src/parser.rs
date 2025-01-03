use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline},
    combinator::{eof, opt},
    multi::many1,
    sequence::delimited,
    IResult,
};

use crate::structs::*;

fn nom_parser(input: &str) -> IResult<&str, PuzzleInput> {
    let (input, packets) = many1(parse_packet)(input)?;
    let (input, _) = eof(input)?;

    Ok((input, PuzzleInput { packets }))
}

pub fn parse(input: &str) -> PuzzleInput {
    nom_parser(input).unwrap().1
}

fn parse_packet(input: &str) -> IResult<&str, (List, List)> {
    let (input, first) = parse_list(input)?;
    let (input, _) = newline(input)?;
    let (input, second) = parse_list(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = opt(newline)(input)?;

    Ok((input, (first, second)))
}

fn parse_list(input: &str) -> IResult<&str, List> {
    let (input, empty) = opt(tag("[],"))(input)?;

    if empty.is_some() {
        return Ok((input, List::Nil));
    }

    let (input, empty) = opt(tag("[]"))(input)?;

    if empty.is_some() {
        return Ok((input, List::Nil));
    }

    let (input, digit) = opt(digit1)(input)?;
    let (input, _) = opt(tag(","))(input)?;

    if let Some(digit) = digit {
        return Ok((input, List::Digit(digit.parse().unwrap())));
    }

    let (input, lists) = delimited(tag("["), many1(parse_list), tag("]"))(input)?;
    let (input, _) = opt(tag(","))(input)?;
    let result = List::Nested(lists.into_iter().collect());

    Ok((input, result))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_parse() {
        assert!(nom_parser(TEST_INPUT).is_ok());
    }
}
