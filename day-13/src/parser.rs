use nom::{
    bytes::complete::tag,
    character::complete::{newline, one_of},
    combinator::eof,
    multi::{many1, separated_list1},
    IResult,
};

use utility_belt::prelude::Grid2D;

use crate::structs::*;

pub fn parse(input: &str) -> PuzzleInput {
    nom_parser(input).unwrap().1
}

pub fn nom_parser(input: &str) -> IResult<&str, PuzzleInput> {
    let (input, patterns) = separated_list1(tag("\n\n"), mirror_pattern)(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = eof(input)?;

    Ok((input, PuzzleInput { patterns }))
}

fn mirror_pattern(input: &str) -> IResult<&str, MirrorPattern> {
    let (input, lines) = separated_list1(tag("\n"), many1(one_of("#.")))(input)?;

    let pattern = Grid2D::from_shape_vec(
        lines[0].len(),
        lines.len(),
        lines.into_iter().flatten().collect(),
    );

    Ok((input, pattern))
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = indoc! {"
        #.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.

        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#
    "};

    #[test]
    fn test_parse() {
        assert!(dbg!(nom_parser(TEST_INPUT)).is_ok());
    }
}
