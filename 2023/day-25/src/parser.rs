use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{alpha1, newline, space0, space1},
    combinator::eof,
    multi::{many1, separated_list1},
    IResult,
};
use ::petgraph::graph::UnGraph;
use utility_belt::prelude::*;

use crate::structs::*;

fn nom_parser(input: &str) -> IResult<&str, PuzzleInput> {
    let (input, edges) = many1(parse_line)(input)?;
    let (input, _) = eof(input)?;

    let mut graph = UnGraph::<String, ()>::new_undirected();
    let mut ids = HashMap::default();

    for (name, connections) in edges.iter() {
        let node = *ids
            .entry(name)
            .or_insert_with(|| graph.add_node(name.clone()));

        for connection in connections {
            let connection = *ids
                .entry(connection)
                .or_insert_with(|| graph.add_node(connection.clone()));

            graph.add_edge(node, connection, ());
        }
    }

    // println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

    Ok((input, PuzzleInput { graph }))
}

pub fn parse(input: &str) -> PuzzleInput {
    nom_parser(input).unwrap().1
}

fn parse_line(input: &str) -> IResult<&str, (String, Vec<String>)> {
    let (input, name) = take_until(": ")(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, _space) = space0(input)?;
    let (input, connections) = separated_list1(space1, alpha1)(input)?;
    let (input, _newline) = newline(input)?;

    Ok((
        input,
        (
            name.to_string(),
            connections.iter().map(|s| s.to_string()).collect(),
        ),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::indoc;

    const TEST_INPUT: &str = indoc! {"
        jqt: rhn xhk nvd
        rsh: frs pzl lsr
        xhk: hfx
        cmg: qnr nvd lhk bvb
        rhn: xhk bvb hfx
        bvb: xhk hfx
        pzl: lsr hfx nvd
        qnr: nvd
        ntq: jqt hfx bvb xhk
        nvd: lhk
        lsr: lhk
        rzs: qnr cmg lsr rsh
        frs: qnr lhk lsr
    "};

    #[test]
    fn test_parse() {
        assert!(nom_parser(TEST_INPUT).is_ok());
    }
}
