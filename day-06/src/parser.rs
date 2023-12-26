use nom::{
    bytes::complete::tag,
    character::complete::{newline, space0, space1},
    combinator::eof,
    multi::separated_list1,
    IResult,
};

use utility_belt::prelude::*;

use crate::structs::*;

fn nom_parser(input: &str) -> IResult<&str, PuzzleInput> {
    let (input, times) = parse_times(input)?;
    let (input, distances) = parse_distances(input)?;
    let (input, _) = eof(input)?;

    Ok((
        input,
        PuzzleInput {
            races: times
                .into_iter()
                .zip(distances)
                .map(|(t, d)| Race {
                    time: t,
                    distance: d,
                })
                .collect::<Vec<_>>(),
        },
    ))
}

fn parse_times(input: &str) -> IResult<&str, Vec<usize>> {
    let (input, _) = tag("Time:")(input)?;
    let (input, _) = space1(input)?;
    let (input, times) = separated_list1(space1, parse_usize)(input)?;
    let (input, _) = newline(input)?;

    Ok((input, times))
}

fn parse_distances(input: &str) -> IResult<&str, Vec<usize>> {
    let (input, _) = tag("Distance:")(input)?;
    let (input, _) = space1(input)?;
    let (input, distances) = separated_list1(space1, parse_usize)(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = space0(input)?;

    Ok((input, distances))
}

pub fn parse(input: &str) -> PuzzleInput {
    nom_parser(input).unwrap().1
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::indoc;

    const TEST_INPUT: &str = indoc! {"
        Time:      7  15   30
        Distance:  9  40  200
    "};

    #[test]
    fn test_parse() {
        assert!(nom_parser(TEST_INPUT).is_ok());
    }
}
