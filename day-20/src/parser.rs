use std::collections::BTreeMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline, one_of},
    combinator::eof,
    multi::{many1, separated_list1},
    IResult,
};
use petgraph::graph::DiGraph;
use utility_belt::prelude::*;

use crate::structs::*;

fn nom_parser(input: &str) -> IResult<&str, PuzzleInput> {
    let (input, modules) = many1(parse_module)(input)?;
    let (input, _) = eof(input)?;

    let mut ids = HashMap::default();
    let mut graph = DiGraph::new();
    let mut node_types = BTreeMap::default();

    for (name, r#type, destinations) in modules.into_iter() {
        node_types.insert(name.clone(), r#type);

        let cur = *ids
            .entry(name.clone())
            .or_insert_with(|| graph.add_node(name));

        for destination in destinations {
            let dest = *ids
                .entry(destination.clone())
                .or_insert_with(|| graph.add_node(destination));

            graph.add_edge(cur, dest, ());
        }
    }

    Ok((input, PuzzleInput { graph, node_types }))
}

pub fn parse(input: &str) -> PuzzleInput {
    nom_parser(input).unwrap().1
}

fn parse_module(input: &str) -> IResult<&str, (String, char, Vec<String>)> {
    let (input, r#type) = parse_module_type(input)?;
    let (input, name) = parse_module_name(input)?;
    let (input, _) = tag(" -> ")(input)?;
    let (input, destinations) = parse_module_destinations(input)?;
    let (input, _) = newline(input)?;

    Ok((input, (name, r#type, destinations)))
}

fn parse_module_type(input: &str) -> IResult<&str, char> {
    one_of("b&%")(input)
}

fn parse_module_name(input: &str) -> IResult<&str, String> {
    let (input, name) = alpha1(input)?;
    Ok((input, name.to_string()))
}

fn parse_module_destinations(input: &str) -> IResult<&str, Vec<String>> {
    let (input, destinations) = separated_list1(tag(", "), alpha1)(input)?;
    let destinations = destinations.into_iter().map(|s| s.to_string()).collect();

    Ok((input, destinations))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = indoc! {"
        broadcaster -> a
        %a -> inv, con
        &inv -> b
        %b -> con
        &con -> output
    "};

    #[test]
    fn test_parse() {
        assert!(nom_parser(TEST_INPUT).is_ok());
    }

    #[test]
    fn tst_foo() {
        let input = crate::parser::parse(TEST_INPUT);
        State::new(&input);
    }
}
