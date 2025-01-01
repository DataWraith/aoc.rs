use utility_belt::prelude::*;

use crate::parser::*;

// The canonical wiring for the segment displays of the DIGITS 0-9
const DIGITS: [&str; 10] = [
    "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
];

// Whether a digit's wires are a superset of another digit's wires. For example,
// 7 is a superset of 1, because all wires of 1 are also lit in 7.
const IS_SUPERSET: [(usize, usize); 32] = [
    (0, 0),
    (1, 1),
    (2, 2),
    (3, 3),
    (4, 4),
    (5, 5),
    (6, 6),
    (7, 7),
    (8, 8),
    (9, 9),
    (0, 1),
    (0, 7),
    (3, 1),
    (3, 7),
    (4, 1),
    (6, 5),
    (7, 1),
    (8, 0),
    (8, 1),
    (8, 2),
    (8, 3),
    (8, 4),
    (8, 5),
    (8, 6),
    (8, 7),
    (8, 8),
    (8, 9),
    (9, 1),
    (9, 3),
    (9, 4),
    (9, 5),
    (9, 7),
];

pub fn part2(input: &PuzzleInput) -> String {
    let mut sum = 0;

    for segments in input.segments.iter() {
        let decoded_wires = decode(segments);

        // Convert the decoded wires to base-10 digits. Because we sorted the
        // wires in the parser, this step is easy.
        let decoded_digits = segments
            .output
            .iter()
            .map(|d| decoded_wires.iter().position(|c| c == d).unwrap())
            .collect::<Vec<_>>();

        // Convert the decoded digits to a base-10 number
        sum += decoded_digits.iter().fold(0, |acc, d| acc * 10 + d);
    }

    sum.to_string()
}

pub fn decode(segments: &Segments) -> Vec<String> {
    let segment_wires = segments
        .calibration
        .iter()
        .map(|d| d.chars().collect::<HashSet<_>>())
        .collect::<Vec<_>>();

    // Initialize a vector of 10 candidates, each being the full set of possible
    // digits.
    let mut candidates = vec![HashSet::from_iter(0..=9); 10];

    // Prune candidates based on the number of lit wires
    for (i, digit) in DIGITS.iter().enumerate() {
        for j in 0..10 {
            if segment_wires[j].len() != digit.len() {
                candidates[j].remove(&i);
            }
        }
    }

    // Prune candidates based on the `IS_SUPERSET` relationship between digits
    for _ in 0..6 {
        for pair in (0..10).combinations(2) {
            let (mut idx1, mut idx2) = (pair[0], pair[1]);

            // If neither candidate is uniquely determined, skip this pair.
            if candidates[idx1].len() != 1 && candidates[idx2].len() != 1 {
                continue;
            }

            // Swap the indices so that idx1 is the uniquely determined candidate
            if candidates[idx1].len() != 1 {
                std::mem::swap(&mut idx1, &mut idx2);
            }

            // Extract the digit and its wires from the first candidate
            let digit = candidates[idx1].iter().cloned().next().unwrap();
            let wires = &segment_wires[idx1];

            // Prune the second candidate based on the `IS_SUPERSET` relationship
            candidates[idx2].retain(|candidate| {
                let wires2 = &segment_wires[idx2];

                if IS_SUPERSET.contains(&(*candidate, digit)) != wires2.is_superset(wires) {
                    return false;
                }

                if IS_SUPERSET.contains(&(digit, *candidate)) != wires.is_superset(wires2) {
                    return false;
                }

                true
            });
        }
    }

    // Convert the uniquely determined candidates to a Vec<String>, with each
    // String representing the lit wires for the corresponding digit.
    candidates
        .iter()
        .map(|c| c.iter().next().unwrap())
        .zip(segments.calibration.iter())
        .sorted()
        .map(|(_idx, wires)| wires.to_string())
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = indoc! {"
        be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
    "};

    #[test]
    fn test_part2_example() {
        let input = crate::parser::part2(TEST_INPUT);
        assert_ne!(TEST_INPUT.trim(), "TODO");
        assert_eq!(part2(&input), "61229");
    }
}
