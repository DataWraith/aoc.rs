use glam::I64Vec3;
use nom::{
    bytes::complete::tag,
    character::complete::{newline, space0, space1},
    combinator::eof,
    multi::many1,
    sequence::separated_pair,
    IResult,
};

use crate::structs::*;

fn nom_parser(input: &str) -> IResult<&str, PuzzleInput> {
    let (input, hailstones) = many1(parse_hailstone)(input)?;
    let (input, _) = eof(input)?;

    Ok((input, PuzzleInput { hailstones }))
}

pub fn parse(input: &str) -> PuzzleInput {
    nom_parser(input).unwrap().1
}

pub fn parse_hailstone(input: &str) -> IResult<&str, Hailstone> {
    let (input, (pos, velocity)) = separated_pair(parse_triplet, tag("@"), parse_triplet)(input)?;
    let (input, _) = newline(input)?;

    Ok((
        input,
        Hailstone {
            position: pos,
            velocity,
        },
    ))
}

pub fn parse_triplet(input: &str) -> IResult<&str, I64Vec3> {
    let (input, _) = space0(input)?;
    let (input, first) = nom::character::complete::i64(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, _) = space1(input)?;
    let (input, second) = nom::character::complete::i64(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, _) = space1(input)?;
    let (input, third) = nom::character::complete::i64(input)?;
    let (input, _) = space0(input)?;

    let vec = I64Vec3::new(first, second, third);

    Ok((input, vec))
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = indoc! {"
        19, 13, 30 @ -2,  1, -2
        18, 19, 22 @ -1, -1, -2
        20, 25, 34 @ -2, -2, -4
        12, 31, 28 @ -1, -2, -1
        20, 19, 15 @  1, -5, -3
    "};

    #[test]
    fn test_parse() {
        assert!(nom_parser(TEST_INPUT).is_ok());
    }
}
