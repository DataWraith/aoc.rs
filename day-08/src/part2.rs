use crate::structs::*;

use utility_belt::prelude::*;

pub fn part2(input: &PuzzleInput) -> String {
    let positions = input
        .nodes
        .iter()
        .filter(|n| n.0.ends_with('A'))
        .map(|n| n.0.clone())
        .collect_vec();

    nsteps(input, &positions)
        .iter()
        .fold(1, |acc, x| lcm(acc, *x))
        .to_string()
}

// NOTE: This won't work in general, but the input is benign here.
pub fn nsteps(input: &PuzzleInput, positions: &[String]) -> Vec<usize> {
    let mut results = Vec::new();

    for p in positions.iter() {
        let mut steps = 0;
        let mut cur = p.clone();

        for instr in input.instructions() {
            if cur.ends_with('Z') {
                break;
            }

            cur = if instr == 'L' {
                input.nodes[&cur].0.clone()
            } else {
                input.nodes[&cur].1.clone()
            };

            steps += 1;
        }

        results.push(steps);
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::indoc;

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
    fn test_part2() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part2(&input), "6");
    }
}
