use utility_belt::prelude::*;

use crate::parser::*;

// This is a bit ugly, but since we're working with static string slices, this is the easiest
// way to get the names of the x, y and z wires without upsetting the borrow checker.
const X_NAMES: [&str; 45] = [
    "x00", "x01", "x02", "x03", "x04", "x05", "x06", "x07", "x08", "x09", "x10", "x11", "x12",
    "x13", "x14", "x15", "x16", "x17", "x18", "x19", "x20", "x21", "x22", "x23", "x24", "x25",
    "x26", "x27", "x28", "x29", "x30", "x31", "x32", "x33", "x34", "x35", "x36", "x37", "x38",
    "x39", "x40", "x41", "x42", "x43", "x44",
];

const Y_NAMES: [&str; 45] = [
    "y00", "y01", "y02", "y03", "y04", "y05", "y06", "y07", "y08", "y09", "y10", "y11", "y12",
    "y13", "y14", "y15", "y16", "y17", "y18", "y19", "y20", "y21", "y22", "y23", "y24", "y25",
    "y26", "y27", "y28", "y29", "y30", "y31", "y32", "y33", "y34", "y35", "y36", "y37", "y38",
    "y39", "y40", "y41", "y42", "y43", "y44",
];

const Z_NAMES: [&str; 46] = [
    "z00", "z01", "z02", "z03", "z04", "z05", "z06", "z07", "z08", "z09", "z10", "z11", "z12",
    "z13", "z14", "z15", "z16", "z17", "z18", "z19", "z20", "z21", "z22", "z23", "z24", "z25",
    "z26", "z27", "z28", "z29", "z30", "z31", "z32", "z33", "z34", "z35", "z36", "z37", "z38",
    "z39", "z40", "z41", "z42", "z43", "z44", "z45",
];

// https://www.youtube.com/watch?v=SU6lp6wyd3I
//
// See also: https://en.wikipedia.org/wiki/Carry-lookahead_adder#/media/File:Partial_Full-Adder.svg
//
// We simply follow the circuit diagram linked above and verify that the
// computation graph matches it.
pub fn verify_z(
    formulas: &HashMap<&str, (&'static str, &'static str, &'static str)>,
    wire: &str,
    num: usize,
) -> bool {
    let (op, left, right) = formulas.get(wire).unwrap();

    // The output (S) must be computed via an XOR gate
    if *op != "XOR" {
        return false;
    }

    // Base case for the z00 wire
    if num == 0 {
        return *left == "x00" && *right == "y00";
    }

    // If the output gate is valid by itself, we need to recursively verify that the "ingredients"
    // are valid:
    //
    // - One of the inputs must have been the sum of the corresponding x and y wires.
    // - The other input must have been a carry bit from the previous step(s).
    (verify_generates_sum(formulas, left, num) && verify_carry_bit(formulas, right, num))
        || (verify_generates_sum(formulas, right, num) && verify_carry_bit(formulas, left, num))
}

// This just checks that the sum of the two current input bits is computed correctly.
pub fn verify_generates_sum(
    formulas: &HashMap<&str, (&'static str, &'static str, &'static str)>,
    wire: &str,
    num: usize,
) -> bool {
    if !formulas.contains_key(wire) {
        return false;
    }

    let (op, left, right) = formulas.get(wire).unwrap();

    // The bitwise sum must be computed via an XOR gate
    if *op != "XOR" {
        return false;
    }

    // And the sum must be the XOR of the corresponding x and y wires.
    *left == X_NAMES[num] && *right == Y_NAMES[num]
}

// The carry bit is computed from the previous step's sum and carry bit, as well
// as the potentially propagated carry bit from the step before that.
//
// Wikipedia denotes this as C_{i+1} = G_i OR (P_i AND C_i),
//
pub fn verify_carry_bit(
    formulas: &HashMap<&str, (&'static str, &'static str, &'static str)>,
    wire: &str,
    num: usize,
) -> bool {
    let (op, left, right) = formulas.get(wire).unwrap();

    // The carry bit for the second output wire is computed as the AND of the
    // x00 and y00 wires (base case).
    if num == 1 {
        if *op != "AND" {
            return false;
        }

        return *left == "x00" && *right == "y00";
    }

    // Otherwise, it must be computed as an OR: Either the previous addition generated a carry bit,
    // or there was a carry bit from the previous step that was propagated.
    if *op != "OR" {
        return false;
    }

    (verify_generated_carry(formulas, left, num) && verify_propagated_carry(formulas, right, num))
        || (verify_generated_carry(formulas, right, num)
            && verify_propagated_carry(formulas, left, num))
}

// The directly generated carry is easy to verify: It must be the AND of the
// previous x-wire and the previous y-wire.
fn verify_generated_carry(
    formulas: &HashMap<&str, (&'static str, &'static str, &'static str)>,
    wire: &str,
    num: usize,
) -> bool {
    let (op, left, right) = formulas.get(wire).unwrap();

    if *op != "AND" {
        return false;
    }

    *left == X_NAMES[num - 1] && *right == Y_NAMES[num - 1]
}

// The recarry bit must be the AND of the previous carry bit and the propagation
// result from the previous step. The propagation result is also the XOR of the
// previous x and y wires, which means we can re-use our verify_generates_sum
// function.
fn verify_propagated_carry(
    formulas: &HashMap<&str, (&'static str, &'static str, &'static str)>,
    wire: &str,
    num: usize,
) -> bool {
    let (op, left, right) = formulas.get(wire).unwrap();

    if *op != "AND" {
        return false;
    }

    (verify_generates_sum(formulas, left, num - 1) && verify_carry_bit(formulas, right, num - 1))
        || (verify_generates_sum(formulas, right, num - 1)
            && verify_carry_bit(formulas, left, num - 1))
}

// Heuristic that returns the number of z-wires that we can verify working.
// Note that we can never verify the 45th z-wire, because that is the
// most-significant bit and not computed the same way as the others.
pub fn heuristic(formulas: &HashMap<&str, (&'static str, &'static str, &'static str)>) -> usize {
    for (z, z_name) in Z_NAMES.iter().enumerate() {
        if !verify_z(formulas, z_name, z) {
            return z;
        }
    }

    unreachable!()
}

pub fn part2(input: &PuzzleInput) -> String {
    let mut formulas = input.formulas.clone();
    let mut baseline = heuristic(&formulas);
    let mut swaps = Vec::new();

    // Brute force search for the best swaps.
    'outer: for _ in 0..4 {
        for (k, v) in formulas.iter() {
            for (k2, v2) in formulas.iter() {
                if k == k2 {
                    continue;
                }

                let mut swapped = formulas.clone();
                swapped.insert(k, *v2);
                swapped.insert(k2, *v);

                let score = heuristic(&swapped);

                if score > baseline {
                    println!(
                        "Swapped {:?} and {:?} with improvement {}, current score: {}",
                        k,
                        k2,
                        score - baseline,
                        score
                    );

                    swaps.push(k.to_string());
                    swaps.push(k2.to_string());

                    baseline = score;
                    formulas = swapped;
                    continue 'outer;
                }
            }
        }
    }

    swaps.iter().sorted().join(",")
}
