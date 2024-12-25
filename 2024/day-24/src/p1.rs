use utility_belt::prelude::*;

use crate::parser::*;

pub fn part1(input: &PuzzleInput) -> String {
    let result = simulate(input);

    readout(&result).to_string()
}

// Derives the resulting integer value from the output wires by bitwise
// concatenation.
fn readout(output_values: &HashMap<&str, bool>) -> usize {
    output_values
        .iter()
        .sorted()
        .rev()
        .fold(0, |acc, x| acc << 1 | *x.1 as usize)
}

pub fn simulate(input: &PuzzleInput) -> HashMap<&str, bool> {
    let mut input = input.clone();
    let mut output_values = HashMap::new();

    let mut q = VecDeque::new();

    for input in input.circuit.neighbors(GateType::Origin) {
        q.push_back(input);
    }

    'outer: while let Some(gate) = q.pop_front() {
        if output_values.len() == input.num_outputs {
            break;
        }

        let mut gate_inputs = Vec::new();

        // Pull inputs from upstream gates/wires. If no input is available yet,
        // push the gate back into the queue to wait for it to be computed.
        for upstream in input
            .circuit
            .edges_directed(gate, petgraph::Direction::Incoming)
        {
            let (_, _, weight) = upstream;

            if let Some(weight) = weight {
                gate_inputs.push(weight);
            } else {
                q.push_back(gate);
                continue 'outer;
            }
        }

        let result = match gate {
            GateType::Origin => {
                panic!("Origin gate should not be in the queue");
            }

            // A Wire just passes through its input.
            GateType::Wire(wire) => {
                assert!(gate_inputs.len() == 1);

                // If the wire is a z-wire, we need to store the result.
                if wire.starts_with("z") {
                    output_values.insert(wire, *gate_inputs[0]);
                }

                *gate_inputs[0]
            }

            // An AND gate computes the bitwise AND of its two inputs.
            GateType::And(_) => {
                assert!(gate_inputs.len() == 2);
                *gate_inputs[0] && *gate_inputs[1]
            }

            // An OR gate computes the bitwise OR of its two inputs.
            GateType::Or(_) => {
                assert!(gate_inputs.len() == 2);
                *gate_inputs[0] || *gate_inputs[1]
            }

            // An XOR gate computes the bitwise XOR of its two inputs.
            GateType::Xor(_) => {
                assert!(gate_inputs.len() == 2);
                *gate_inputs[0] ^ *gate_inputs[1]
            }
        };

        let mut neighbors = Vec::new();

        for neighbor in input
            .circuit
            .neighbors_directed(gate, petgraph::Direction::Outgoing)
        {
            neighbors.push(neighbor);
        }

        // Propagate the result to downstream gates/wires and add them to the
        // queue for processing.
        for neighbor in neighbors {
            if let Some(w) = input.circuit.edge_weight_mut(gate, neighbor) {
                *w = Some(result);
            }

            q.push_back(neighbor);
        }
    }

    output_values
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = indoc! {"
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
"};

    #[test]
    fn test_part1_example() {
        let input = crate::parser::part1(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(part1(&input), "2024");
    }
}
