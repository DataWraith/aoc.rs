use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline, space0},
    combinator::eof,
    multi::{separated_list1, many1},
    IResult,
};

use crate::structs::*;

fn nom_parser(input: &str) -> IResult<&str, PuzzleInput> {
    let (input, games) = many1(parse_game)(input)?;
    let (input, _) = eof(input)?;

    Ok((input, PuzzleInput { games }))
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game ")(input)?;
    let (input, id) = digit1(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, cubes) = parse_subsets(input)?;
    let (input, _) = newline(input)?;

    Ok((
        input,
        Game {
            id: id.parse().unwrap(),
            cubes,
        },
    ))
}

fn parse_subsets(input: &str) -> IResult<&str, Vec<Cubes>> {
    let (input, subsets) = separated_list1(tag("; "), parse_cubes)(input)?;
    Ok((input, subsets))
}

fn parse_cubes(input: &str) -> IResult<&str, Cubes> {
    let (input, cubes) = separated_list1(tag(", "), parse_cube)(input)?;
    Ok((input, cubes.into_iter().sum()))
}

fn parse_cube(input: &str) -> IResult<&str, Cubes> {
    let (input, _) = space0(input)?;
    let (input, count) = digit1(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, color) = parse_color(input)?;

    match color {
        "red" => Ok((
            input,
            Cubes {
                red: count.parse().unwrap(),
                ..Default::default()
            },
        )),
        "green" => Ok((
            input,
            Cubes {
                green: count.parse().unwrap(),
                ..Default::default()
            },
        )),
        "blue" => Ok((
            input,
            Cubes {
                blue: count.parse().unwrap(),
                ..Default::default()
            },
        )),
        _ => unreachable!(),
    }
}

fn parse_color(input: &str) -> IResult<&str, &str> {
    let (input, color) = nom::branch::alt((
        nom::bytes::complete::tag("red"),
        nom::bytes::complete::tag("green"),
        nom::bytes::complete::tag("blue"),
    ))(input)?;

    Ok((input, color))
}

pub fn parse(input: &str) -> PuzzleInput {
    nom_parser(input).unwrap().1
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = indoc! {"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    "};

    #[test]
    fn test_parse() {
        assert!(nom_parser(TEST_INPUT).is_ok());
    }
}
