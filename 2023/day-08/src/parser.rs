use std::collections::BTreeMap;

use nom::{
    bytes::complete::{tag, take_until},
    combinator::eof,
    IResult,
};

use crate::structs::*;

fn nom_parser(input: &str) -> IResult<&str, PuzzleInput> {
    let instructions = input.lines().next().unwrap();
    let (input, network) = parse_network(input).expect("Failed to parse network");

    Ok((
        input,
        PuzzleInput {
            instructions: instructions.to_string(),
            nodes: network,
        },
    ))
}

fn parse_network(input: &str) -> IResult<&str, BTreeMap<String, (String, String)>> {
    let (input, _) = take_until("\n\n")(input)?;
    let (input, _) = tag("\n\n")(input)?;
    let (input, nodes) = nom::multi::separated_list1(tag("\n"), parse_node)(input)?;
    let (input, _) = tag("\n")(input)?;
    let (input, _) = eof(input)?;

    let mut map = BTreeMap::new();

    for (key, value) in nodes.into_iter() {
        map.insert(key, value);
    }

    Ok((input, map))
}

fn parse_node(input: &str) -> IResult<&str, (String, (String, String))> {
    let (input, name) = take_until(" = ")(input)?;
    let (input, _) = tag(" = (")(input)?;
    let (input, left) = take_until(", ")(input)?;
    let (input, _) = tag(", ")(input)?;
    let (input, right) = take_until(")")(input)?;
    let (input, _) = tag(")")(input)?;

    Ok((
        input,
        (name.to_string(), (left.to_string(), right.to_string())),
    ))
}

pub fn parse(input: &str) -> PuzzleInput {
    nom_parser(input).unwrap().1
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = indoc! {"
        LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)
    "};

    #[test]
    fn test_parse() {
        assert!(nom_parser(TEST_INPUT).is_ok());
    }
}
