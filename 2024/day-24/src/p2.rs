use std::hash::Hash;

use indicatif::{ProgressIterator, ProgressStyle};
use petgraph::{
    algo::{condensation, dijkstra},
    dot::Dot,
    graph::NodeIndex,
    prelude::DiGraphMap,
    visit::IntoEdgesDirected,
};
use rand::prelude::*;
use utility_belt::prelude::*;

use crate::parser::*;

const X_NAMES: [&'static str; 45] = [
    "x00", "x01", "x02", "x03", "x04", "x05", "x06", "x07", "x08", "x09", "x10", "x11", "x12",
    "x13", "x14", "x15", "x16", "x17", "x18", "x19", "x20", "x21", "x22", "x23", "x24", "x25",
    "x26", "x27", "x28", "x29", "x30", "x31", "x32", "x33", "x34", "x35", "x36", "x37", "x38",
    "x39", "x40", "x41", "x42", "x43", "x44",
];

const Y_NAMES: [&'static str; 45] = [
    "y00", "y01", "y02", "y03", "y04", "y05", "y06", "y07", "y08", "y09", "y10", "y11", "y12",
    "y13", "y14", "y15", "y16", "y17", "y18", "y19", "y20", "y21", "y22", "y23", "y24", "y25",
    "y26", "y27", "y28", "y29", "y30", "y31", "y32", "y33", "y34", "y35", "y36", "y37", "y38",
    "y39", "y40", "y41", "y42", "y43", "y44",
];

const Z_NAMES: [&'static str; 46] = [
    "z00", "z01", "z02", "z03", "z04", "z05", "z06", "z07", "z08", "z09", "z10", "z11", "z12",
    "z13", "z14", "z15", "z16", "z17", "z18", "z19", "z20", "z21", "z22", "z23", "z24", "z25",
    "z26", "z27", "z28", "z29", "z30", "z31", "z32", "z33", "z34", "z35", "z36", "z37", "z38",
    "z39", "z40", "z41", "z42", "z43", "z44", "z45",
];

pub fn part2(input: &PuzzleInput) -> String {
    // I suspected that the circuit is a ripple-carry adder, so I looked at the Wikipedia page and came up with
    // the following invariants:
    //
    // 1. Xor gates either feed into an Output wire or into another Xor gate
    // 2. Output wires are fed by an XOR gate combining two inputs
    // 3. AND gates take inputs from X or Y wires and output to OR gates
    // 4. OR gates output to XOR gates

    let mut candidates = Vec::new();

    for node in input.circuit.nodes() {
        match node {
            GateType::Xor(wire) | GateType::And(wire) | GateType::Or(wire) => {
                if wire.contains("gjp") {
                    //dbg!(&node);
                }
            }

            _ => {}
        }

        match node {
            GateType::Origin => {
                continue;
            }

            GateType::Xor(wire) => {
                'outer: for neighbor in input
                    .circuit
                    .edges_directed(node, petgraph::Direction::Outgoing)
                {
                    let (_, neighbor, _) = neighbor;

                    if wire.contains("x") && !wire.contains("z") {
                        continue;
                    }

                    if wire.contains("z") && !wire.contains("x") {
                        continue;
                    }

                    match neighbor {
                        GateType::Wire(name) => {
                            for neighbor2 in input
                                .circuit
                                .edges_directed(neighbor, petgraph::Direction::Outgoing)
                            {
                                let (_, neighbor2, _) = neighbor2;

                                if let GateType::Xor(_) = neighbor2 {
                                    continue 'outer;
                                }

                                candidates.push(node);
                            }
                        }

                        _ => {
                            candidates.push(node);
                        }
                    }
                }
            }

            GateType::And(wire) => {
                if wire.contains("x") || wire.contains("y") {
                    //continue;
                }

                'outer: for neighbor in input
                    .circuit
                    .edges_directed(node, petgraph::Direction::Outgoing)
                {
                    let (_, neighbor, _) = neighbor;

                    if let GateType::Wire(_) = neighbor {
                        for neighbor2 in input
                            .circuit
                            .edges_directed(neighbor, petgraph::Direction::Outgoing)
                        {
                            let (_, neighbor2, _) = neighbor2;

                            if let GateType::Or(_) = neighbor2 {
                                continue 'outer;
                            }
                        }
                    }

                    candidates.push(node);
                }
            }

            GateType::Or(wire) => {
                'outer: for neighbor in input
                    .circuit
                    .edges_directed(node, petgraph::Direction::Outgoing)
                {
                    let (_, neighbor, _) = neighbor;

                    if let GateType::Wire(name) = neighbor {
                        for neighbor2 in input
                            .circuit
                            .edges_directed(neighbor, petgraph::Direction::Outgoing)
                        {
                            let (_, neighbor2, _) = neighbor2;

                            if let GateType::Xor(_) = neighbor2 {
                                continue 'outer;
                            }
                        }
                    }

                    candidates.push(node);
                }
            }

            _ => {}
        }
    }

    dbg!(&candidates);

    let mut visited = HashSet::new();
    //let mut investigate = Vec::new();
    let mut ctr = 0;
    let mut candidates2 = Vec::new();

    for node in input.circuit.nodes() {
        if !matches!(node, GateType::Xor(_) | GateType::And(_) | GateType::Or(_)) {
            continue;
        }

        if candidates.contains(&node) {
            continue;
        }

        if !candidates2.is_empty() {
            break;
        }

        candidates2 = candidates.clone();

        //candidates2.push(node);

        for comb in candidates2.iter().combinations(8) {
            dbg!(&comb);

            'outer: for perm in comb.into_iter().permutations(8) {
                ctr += 1;

                let a = perm[0].clone();
                let b = perm[1].clone();
                let c = perm[2].clone();
                let d = perm[3].clone();
                let e = perm[4].clone();
                let f = perm[5].clone();
                let g = perm[6].clone();
                let h = perm[7].clone();

                let mut x = [a, b];
                let mut y = [c, d];
                let mut z = [e, f];
                let mut w = [g, h];

                x.sort();
                y.sort();
                z.sort();
                w.sort();

                let mut swaps = [x, y, z, w];
                swaps.sort();

                if visited.contains(&swaps) {
                    continue;
                }

                visited.insert(swaps);

                let mut input2 = input.clone();

                input2 = swap_gates(&input2, a, b);
                input2 = swap_gates(&input2, c, d);
                input2 = swap_gates(&input2, e, f);
                input2 = swap_gates(&input2, g, h);

                let mut rng = StdRng::from_seed([11; 32]);

                for i in 0..10 {
                    let x: usize = rng.gen_range(0..=0xffffffffffff);
                    let y: usize = rng.gen_range(0..=0xffffffffffff);
                    let x = x & 0b111111111111111111111111111111111111111111111;
                    let y = y & 0b111111111111111111111111111111111111111111111;
                    let z = x + y;

                    let (_, _, output_values) = simulate_forward(&input2, x, y);
                    let result = derive_output_value(&output_values);

                    if result != z {
                        if result.abs_diff(z) < 100 {
                            //investigate.push((perm.clone(), z.abs_diff(result)));
                        }

                        dbg!((ctr, result.abs_diff(z)));
                        continue 'outer;
                    }
                }

                dbg!(&perm);
                break;
            }
        }
    }

    //dbg!(&investigate);

    todo!()
}

pub fn swap_gates(input: &PuzzleInput, gate1: GateType, gate2: GateType) -> PuzzleInput {
    let mut input = input.clone();

    let g1_output = input
        .circuit
        .neighbors_directed(gate1, petgraph::Direction::Outgoing)
        .next()
        .unwrap();

    let g2_output = input
        .circuit
        .neighbors_directed(gate2, petgraph::Direction::Outgoing)
        .next()
        .unwrap();

    input.circuit.remove_edge(gate1, g1_output);
    input.circuit.remove_edge(gate2, g2_output);

    input.circuit.add_edge(gate1, g2_output, None);
    input.circuit.add_edge(gate2, g1_output, None);

    input
}

fn derive_output_value(output_values: &HashMap<&str, bool>) -> usize {
    let mut result = 0;
    let mut idx = 0;

    while let Some(value) = output_values.get(Z_NAMES[idx]) {
        result |= (*value as usize) << idx;
        idx += 1;

        if idx == 46 {
            break;
        }
    }

    result
}

pub fn simulate_forward(
    input: &PuzzleInput,
    x: usize,
    y: usize,
) -> (PuzzleInput, HashSet<GateType>, HashMap<&str, bool>) {
    let input = input.clone();
    let mut output_values = HashMap::new();

    let mut x = x;
    let mut x_idx = 0;
    let mut y = y;
    let mut y_idx = 0;

    let mut graph: DiGraphMap<_, _> =
        DiGraphMap::from_graph(input.circuit.clone().into_graph().map(
            |_: NodeIndex<usize>, x| *x,
            |_idx, edge| (*edge, HashSet::new()),
        ));

    loop {
        let bit = x & 1;

        if let Some(name) = X_NAMES.get(x_idx) {
            if let Some(weight) = graph.edge_weight_mut(GateType::Origin, GateType::Wire(name)) {
                *weight = (Some(bit == 1), HashSet::from([x_idx]));
            }
        }

        x >>= 1;
        x_idx += 1;

        if x == 0 && x_idx > input.num_outputs {
            break;
        }
    }

    loop {
        let bit = y & 1;

        if let Some(name) = Y_NAMES.get(y_idx) {
            if let Some(weight) = graph.edge_weight_mut(GateType::Origin, GateType::Wire(name)) {
                *weight = (Some(bit == 1), HashSet::from([y_idx]));
            }
        }

        y >>= 1;
        y_idx += 1;

        if y == 0 && y_idx > input.num_outputs {
            break;
        }
    }

    let mut counter = 0;

    let mut q = VecDeque::new();

    for input in graph.neighbors(GateType::Origin) {
        q.push_back(input);
    }

    let mut tracer_set = HashSet::new();

    'outer: while let Some(gate) = q.pop_front() {
        if output_values.len() == input.num_outputs {
            break;
        }

        counter += 1;

        if counter > 1_000_000 {
            return (input, HashSet::new(), HashMap::new());
        }

        let mut gate_inputs = Vec::new();

        for upstream in graph.edges_directed(gate, petgraph::Direction::Incoming) {
            let (_, _, weight) = upstream;

            if let (Some(value), origin) = weight {
                gate_inputs.push(weight);
            } else {
                q.push_back(gate);
                continue 'outer;
            }
        }

        //let tracer_intersection = gate_inputs
        //    .iter()
        //    .map(|(_, origin)| origin.clone())
        //    .reduce(|a, b| a.intersection(&b).map(|x| *x).collect::<HashSet<_>>())
        //    .unwrap();

        //let tracer = gate_inputs
        //    .iter()
        //    .map(|(_, origin)| origin.clone())
        //    .reduce(|a, b| a.union(&b).map(|x| *x).collect::<HashSet<_>>())
        //    .unwrap();

        //if tracer.len() > 0 && tracer.len() != tracer_intersection.len() {
        //    tracer_set.insert(gate);
        //}

        let result = match gate {
            GateType::Origin => {
                panic!("Origin gate should not be in the queue");
            }

            GateType::Wire(wire) => {
                assert!(gate_inputs.len() == 1);

                if wire.starts_with("z") {
                    output_values.insert(wire, gate_inputs[0].clone());
                }

                gate_inputs[0].0.unwrap()
            }

            GateType::And(idx) => {
                assert!(gate_inputs.len() == 2);
                gate_inputs[0].0.unwrap() && gate_inputs[1].0.unwrap()
            }

            GateType::Or(idx) => {
                assert!(gate_inputs.len() == 2);
                gate_inputs[0].0.unwrap() || gate_inputs[1].0.unwrap()
            }

            GateType::Xor(idx) => {
                assert!(gate_inputs.len() == 2);
                gate_inputs[0].0.unwrap() ^ gate_inputs[1].0.unwrap()
            }
        };

        let result = (Some(result), HashSet::new());

        let mut neighbors = Vec::new();

        for neighbor in graph.neighbors_directed(gate, petgraph::Direction::Outgoing) {
            neighbors.push(neighbor);
        }

        for neighbor in neighbors {
            for w in graph.edge_weight_mut(gate, neighbor) {
                *w = result.clone();
            }

            q.push_back(neighbor);
        }
    }

    //dbg!(&tracer_set, tracer_set.len());

    (
        input,
        tracer_set,
        output_values
            .iter()
            .map(|(k, value)| (*k, value.0.unwrap()))
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = indoc! {"
    "};

    // x00 AND y00 -> z05
    // x01 AND y01 -> z02
    // x02 AND y02 -> z01
    // x03 AND y03 -> z03
    // x04 AND y04 -> z04
    // x05 AND y05 -> z00

    #[test]
    fn test_part2_example() {
        let input = crate::parser::part2(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(part2(&input), "z00,z01,z02,z05");
    }
}
