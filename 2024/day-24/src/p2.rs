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

// https://www.youtube.com/watch?v=SU6lp6wyd3I
pub fn verify_intermediate_xor(
    formulas: &HashMap<&str, (&'static str, &'static str, &'static str)>,
    wire: &str,
    num: usize,
) -> bool {
    if !formulas.contains_key(wire) {
        return false;
    }

    let (op, left, right) = formulas.get(wire).unwrap();

    let (op, left, right) = if left > right {
        (op, right, left)
    } else {
        (op, left, right)
    };

    if *op != "XOR" {
        return false;
    }

    *left == X_NAMES[num] && *right == Y_NAMES[num]
}

pub fn verify_carry_bit(
    formulas: &HashMap<&str, (&'static str, &'static str, &'static str)>,
    wire: &str,
    num: usize,
) -> bool {
    let (op, left, right) = formulas.get(wire).unwrap();
    let (op, left, right) = if left > right {
        (op, right, left)
    } else {
        (op, left, right)
    };

    if num == 1 {
        if *op != "AND" {
            return false;
        }

        return *left == "x00" && *right == "y00";
    }

    if *op != "OR" {
        return false;
    }

    (verify_direct_carry(formulas, left, num - 1) && verify_recarry(formulas, right, num - 1))
        || (verify_direct_carry(formulas, right, num - 1)
            && verify_recarry(formulas, left, num - 1))
}

fn verify_direct_carry(
    formulas: &HashMap<&str, (&'static str, &'static str, &'static str)>,
    wire: &str,
    num: usize,
) -> bool {
    let (op, left, right) = formulas.get(wire).unwrap();
    let (op, left, right) = if left > right {
        (op, right, left)
    } else {
        (op, left, right)
    };

    if *op != "AND" {
        return false;
    }

    *left == X_NAMES[num] && *right == Y_NAMES[num]
}

fn verify_recarry(
    formulas: &HashMap<&str, (&'static str, &'static str, &'static str)>,
    wire: &str,
    num: usize,
) -> bool {
    let (op, left, right) = formulas.get(wire).unwrap();
    let (op, left, right) = if left > right {
        (op, right, left)
    } else {
        (op, left, right)
    };

    if *op != "AND" {
        return false;
    }

    (verify_intermediate_xor(formulas, left, num) && verify_carry_bit(formulas, right, num))
        || (verify_intermediate_xor(formulas, right, num) && verify_carry_bit(formulas, left, num))
}

pub fn verify_z(
    formulas: &HashMap<&str, (&'static str, &'static str, &'static str)>,
    wire: &str,
    num: usize,
) -> bool {
    let (op, left, right) = formulas.get(wire).unwrap();
    let (op, left, right) = if left > right {
        (op, right, left)
    } else {
        (op, left, right)
    };

    if *op != "XOR" {
        return false;
    }

    if num == 0 {
        return *left == "x00" && *right == "y00";
    }

    (verify_intermediate_xor(formulas, left, num) && verify_carry_bit(formulas, right, num))
        || (verify_intermediate_xor(formulas, right, num) && verify_carry_bit(formulas, left, num))
}

pub fn part2(input: &PuzzleInput) -> String {
    for (k, v) in input.formulas.iter() {
        for (k2, v2) in input.formulas.iter() {
            if k2 == k {
                continue;
            }

            let mut formulas = input.formulas.clone();

            if k.starts_with("x") || k.starts_with("y") {
                continue;
            }

            if k2.starts_with("x") || k2.starts_with("y") {
                continue;
            }

            if *k == "fkp"
                || *k == "z06"
                || *k == "z11"
                || *k == "ngr"
                || *k == "z31"
                || *k == "mfm"
            {
                continue;
            }

            formulas.insert("fkp", ("OR", "bpp", "ghf"));
            formulas.insert("z06", ("XOR", "wvr", "jgw"));
            formulas.insert("z11", ("XOR", "jpp", "stv"));
            formulas.insert("ngr", ("AND", "stv", "jpp"));
            formulas.insert("z31", ("XOR", "mgq", "tpf"));
            formulas.insert("mfm", ("AND", "y31", "x31"));

            formulas.insert(k, *v2);
            formulas.insert(k2, *v);

            let mut found = false;

            for z in 0..=44 {
                if !verify_z(&formulas, Z_NAMES[z], z) {
                    break;
                }

                if z == 44 {
                    found = true;
                    break;
                }
            }

            if found {
                dbg!("FOUND");
                dbg!(k, v, k2, v2);
                panic!();
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

    (
        input,
        tracer_set,
        output_values
            .iter()
            .map(|(k, value)| (*k, value.0.unwrap()))
            .collect(),
    )
}
