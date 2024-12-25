use petgraph::{graph::DiGraph, prelude::DiGraphMap};
use utility_belt::prelude::*;

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub inputs: HashMap<&'static str, bool>,
    pub circuit: DiGraphMap<GateType, Option<bool>>,
    pub num_outputs: usize,
    pub formulas: HashMap<&'static str, (&'static str, &'static str, &'static str)>,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum GateType {
    Origin,
    Wire(&'static str),
    And(&'static str),
    Or(&'static str),
    Xor(&'static str),
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub struct Gate {
    inputs: [Option<bool>; 2],
    gate_type: GateType,
}

pub struct GateConnection {
    inputs: [&'static str; 2],
    output: &'static str,
    gate: Gate,
}

fn parse_gate(
    gate: &'static str,
    formulas: &mut HashMap<&'static str, (&'static str, &'static str, &'static str)>,
) -> GateConnection {
    let (a, gate_type, b, _, out) = gate.splitn(5, ' ').collect_tuple().unwrap();

    formulas.insert(out, (gate_type, a, b));

    let gate_type = match gate_type {
        "AND" => GateType::And(gate),
        "OR" => GateType::Or(gate),
        "XOR" => GateType::Xor(gate),
        _ => panic!("Invalid gate type: {}", gate_type),
    };

    GateConnection {
        inputs: [a, b],
        output: out,
        gate: Gate {
            inputs: [None, None],
            gate_type,
        },
    }
}

pub fn part1(input: &'static str) -> PuzzleInput {
    let mut input_map = HashMap::new();
    let mut graph: DiGraphMap<GateType, Option<bool>> = DiGraphMap::new();
    let mut formulas = HashMap::new();

    let (inputs, gates) = input.split_once("\n\n").unwrap();

    for line in inputs.lines() {
        parse_input(&mut input_map, line.trim());
    }

    let origin = graph.add_node(GateType::Origin);

    for (input, value) in input_map.iter() {
        let input_node = graph.add_node(GateType::Wire(input));
        graph.add_edge(origin, input_node, Some(*value));
    }

    let gate_connections = gates
        .lines()
        .map(|line| parse_gate(line.trim(), &mut formulas))
        .collect_vec();

    let mut num_outputs = 0;

    for gate_connection in gate_connections {
        let gate_node = graph.add_node(gate_connection.gate.gate_type);

        let a = GateType::Wire(gate_connection.inputs[0]);
        let b = GateType::Wire(gate_connection.inputs[1]);
        let o = GateType::Wire(gate_connection.output);

        if gate_connection.output.starts_with("z") {
            num_outputs += 1;
        }

        graph.add_edge(a, gate_node, None);
        graph.add_edge(b, gate_node, None);
        graph.add_edge(gate_node, o, None);
    }

    PuzzleInput {
        inputs: input_map,
        circuit: graph,
        num_outputs,
        formulas,
    }
}

fn parse_input(input_map: &mut HashMap<&'static str, bool>, line: &'static str) {
    let (key, value) = line.split_once(": ").unwrap();

    if value == "1" {
        input_map.insert(key, true);
    } else if value == "0" {
        input_map.insert(key, false);
    } else {
        panic!("Invalid input: {}", value);
    }
}

pub fn part2(input: &'static str) -> PuzzleInput {
    part1(input)
}
