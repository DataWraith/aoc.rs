use crate::structs::*;

use utility_belt::prelude::*;

pub fn part2(input: &PuzzleInput) -> String {
    let start_positions = input
        .nodes
        .iter()
        .filter(|n| n.0.ends_with('A'))
        .map(|n| n.0.clone())
        .collect_vec();

    // NOTE: I'm not sure if this will work in general, or if the input is just
    // benign here.
    //
    // The idea is to step through the instructions, and for each step, update
    // the current positions of all ghosts in parallel. Once all ghosts have
    // reached a destination tile, we can calculate the LCM of all the steps it
    // took to get there to find the first time they will all be on a tile with
    // a Z.
    //
    // However, I think this assumes that the 'rendevous' node is the very first
    // one each ghost reaches, and I'm not sure if this works in the general
    // case.
    let mut results = start_positions.iter().map(|_| None).collect_vec();
    let mut cur = start_positions;

    for (step, instr) in input.instructions().enumerate() {
        cur = cur
            .iter()
            .map(|p| {
                if instr == 'L' {
                    input.nodes[p].0.clone()
                } else {
                    input.nodes[p].1.clone()
                }
            })
            .collect_vec();

        cur.iter().enumerate().for_each(|(i, p)| {
            if p.ends_with('Z') {
                results[i].get_or_insert(step + 1);
            }
        });

        if results.iter().all(Option::is_some) {
            break;
        }
    }

    results
        .iter()
        .fold(1, |acc, r| lcm(acc, r.unwrap()))
        .to_string()
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
