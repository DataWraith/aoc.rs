use nom::{
    bytes::complete::tag,
    character::complete::newline,
    combinator::eof,
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};
use utility_belt::prelude::*;

use crate::{
    bvh::{AABB, BVH},
    structs::*,
};

pub fn parse(input: &str) -> PuzzleInput {
    nom_parser(input).unwrap().1
}

fn nom_parser(input: &str) -> IResult<&str, PuzzleInput> {
    let (input, bricks) = many1(parse_brick)(input)?;
    let (input, _) = eof(input)?;

    let bricks = bricks
        .into_iter()
        .map(|mut brick| {
            let (min_x, max_x) = (
                brick.lower_bound.x.min(brick.upper_bound.x),
                brick.lower_bound.x.max(brick.upper_bound.x),
            );
            let (min_y, max_y) = (
                brick.lower_bound.y.min(brick.upper_bound.y),
                brick.lower_bound.y.max(brick.upper_bound.y),
            );
            let (min_z, max_z) = (
                brick.lower_bound.z.min(brick.upper_bound.z),
                brick.lower_bound.z.max(brick.upper_bound.z),
            );

            brick.lower_bound = IVec3::new(min_x, min_y, min_z);
            brick.upper_bound = IVec3::new(max_x, max_y, max_z);

            brick
        })
        .collect::<Vec<_>>();

    Ok((input, PuzzleInput { bricks }))
}

pub fn parse_brick(input: &str) -> IResult<&str, AABB> {
    let (input, coords) = separated_pair(parse_coordinate, tag("~"), parse_coordinate)(input)?;
    let (input, _) = newline(input)?;
    Ok((
        input,
        AABB {
            lower_bound: coords.0,
            upper_bound: coords.1,
        },
    ))
}

pub fn parse_coordinate(input: &str) -> IResult<&str, IVec3> {
    let (input, coords) = separated_list1(tag(","), nom::character::complete::i32)(input)?;

    let x = coords[0];
    let y = coords[1];
    let z = coords[2];

    Ok((input, IVec3::new(x, y, z)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::indoc;

    const TEST_INPUT: &str = indoc! {"
        1,0,1~1,2,1
        0,0,2~2,0,2
        0,2,3~2,2,3
        0,0,4~0,2,4
        2,0,5~2,2,5
        0,1,6~2,1,6
        1,1,8~1,1,9
    "};

    #[test]
    fn test_parse() {
        assert!(nom_parser(TEST_INPUT).is_ok());
    }
}
