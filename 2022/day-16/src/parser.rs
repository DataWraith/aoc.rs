use ::petgraph::algo::floyd_warshall;

use utility_belt::prelude::{
    nom::{branch::alt, bytes::complete::take_until1, combinator::opt},
    *,
};

use crate::structs::*;

fn nom_parser(input: &str) -> IResult<&str, PuzzleInput> {
    let (input, valves) = many1(parse_line)(input)?;
    let (input, _) = eof(input)?;

    let mut valve_pressures = Vec::new();
    let mut network = petgraph::UnGraph::<u16, u8, u8>::default();
    let mut node_ids = HashMap::default();

    for valve in valves.iter() {
        node_ids
            .entry(valve.0.clone())
            .or_insert_with(|| network.add_node(valve.1));

        if valve.1 > 0 {
            valve_pressures.push((node_ids[&valve.0], valve.1));
        }
    }

    valve_pressures.sort_by_key(|(_id, flow)| std::cmp::Reverse(*flow));

    for valve in valves.iter() {
        let from = node_ids[&valve.0];
        for neighbor in valve.2.iter() {
            let to = node_ids[neighbor];
            network.add_edge(from, to, 1);
        }
    }

    let distances = floyd_warshall(&network, |e| *e.weight()).unwrap();

    Ok((
        input,
        PuzzleInput {
            valve_ids: node_ids,
            valve_pressures,
            distances,
            network,
        },
    ))
}

pub fn parse_line(input: &str) -> IResult<&str, (String, u16, Vec<String>)> {
    let (input, _) = tag("Valve ")(input)?;
    let (input, name) = take_until1(" ")(input)?;
    let (input, _) = tag(" has flow rate=")(input)?;
    let (input, flow_rate) = u16(input)?;
    let (input, _) = alt((
        tag("; tunnel leads to valve "),
        tag("; tunnels lead to valves "),
    ))(input)?;
    let (input, first_neighbor) = alpha1(input)?;
    let (input, _) = opt(tag(", "))(input)?;
    let (input, neighbors) = opt(separated_list0(tag(", "), alpha1))(input)?;
    let (input, _) = newline(input)?;

    Ok((
        input,
        (
            name.to_string(),
            flow_rate,
            std::iter::once(vec![first_neighbor])
                .chain(neighbors)
                .flat_map(|s| s.into_iter().map(|s| s.to_string()))
                .collect(),
        ),
    ))
}

pub fn parse(input: &str) -> PuzzleInput {
    nom_parser(input).unwrap().1
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
