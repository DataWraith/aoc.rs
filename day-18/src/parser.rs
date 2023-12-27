use nom::{
    bytes::complete::tag,
    character::complete::{newline, one_of, space1},
    combinator::eof,
    multi::many1,
    IResult,
};
use utility_belt::prelude::*;

use crate::structs::*;

fn nom_parser(input: &str) -> IResult<&str, PuzzleInput> {
    let (input, plan) = many1(parse_edge)(input)?;
    let (input, _) = eof(input)?;

    Ok((input, PuzzleInput { plan }))
}

pub fn parse(input: &str) -> PuzzleInput {
    nom_parser(input).unwrap().1
}

fn parse_edge(input: &str) -> IResult<&str, PlanEdge> {
    let (input, direction) = parse_direction(input)?;
    let (input, _) = space1(input)?;
    let (input, distance) = parse_usize(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = tag("(")(input)?;
    let (input, color) = parse_hex_color(input)?;
    let (input, _) = tag(")")(input)?;
    let (input, _) = newline(input)?;

    Ok((
        input,
        PlanEdge {
            direction,
            distance,
            color,
        },
    ))
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    let (input, d) = one_of("UDLR")(input)?;
    let d = match d {
        'U' => Direction::Up,
        'D' => Direction::Down,
        'L' => Direction::Left,
        'R' => Direction::Right,
        _ => unreachable!(),
    };

    Ok((input, d))
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::indoc;

    const TEST_INPUT: &str = indoc! {"
        R 6 (#70c710)
        D 5 (#0dc571)
        L 2 (#5713f0)
        D 2 (#d2c081)
        R 2 (#59c680)
        D 2 (#411b91)
        L 5 (#8ceee2)
        U 2 (#caa173)
        L 1 (#1b58a2)
        U 2 (#caa171)
        R 2 (#7807d2)
        U 3 (#a77fa3)
        L 2 (#015232)
        U 2 (#7a21e3)
    "};

    #[test]
    fn test_parse() {
        assert!(nom_parser(TEST_INPUT).is_ok());
    }
}
