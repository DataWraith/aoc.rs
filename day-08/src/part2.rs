use std::collections::VecDeque;

use crate::structs::*;

use utility_belt::prelude::*;

pub fn part2(input: &PuzzleInput) -> String {
    let loop_lengths = input
        .nodes
        .keys()
        .filter(|node| node.ends_with('A'))
        .map(|node| single_loop(input, node.clone()))
        .map(|(cycle_starts_at, cycle)| cycle_times(&cycle, cycle_starts_at))
        .collect::<Vec<_>>();

    (loop_lengths.into_iter().fold(HashSet::default(), |acc, x| {
        let mut result = HashSet::default();

        if acc.is_empty() {
            for x in x.iter() {
                result.insert(*x);
            }
            return result;
        }

        for a in acc.iter() {
            for b in x.iter() {
                result.insert(lcm(*a, *b));
            }
        }

        result
    }))
    .into_iter()
    .min()
    .unwrap()
    .to_string()
}

fn single_loop(input: &PuzzleInput, from: String) -> (usize, Vec<GraphNode>) {
    let instr_len = input.instructions.len();

    let mut current = from;
    let mut instrs = input.instructions();
    let mut index = 0;
    let cycle_start;

    let mut result = VecDeque::new();
    let mut seen = HashSet::default();

    let mut node = GraphNode {
        node: current.clone(),
        index,
    };

    loop {
        result.push_back(node.clone());

        if !seen.insert(node.clone()) {
            cycle_start = node;
            break;
        }

        let go_left = instrs.next().unwrap() == 'L';

        if go_left {
            let (left, _) = input.nodes.get(&current).unwrap();
            current = left.clone()
        } else {
            let (_, right) = input.nodes.get(&current).unwrap();
            current = right.clone()
        }

        index = (index + 1) % instr_len;

        node = GraphNode {
            node: current.clone(),
            index,
        };
    }

    let mut cycle_starts_at = 0;
    while result.front() != Some(&cycle_start) {
        result.pop_front();
        cycle_starts_at += 1;
    }

    result.pop_back();

    (cycle_starts_at, result.into())
}

fn cycle_times(cycle: &[GraphNode], cycle_starts_at: usize) -> Vec<usize> {
    let mut result = Vec::new();

    for (i, node) in cycle.iter().enumerate() {
        if node.node.ends_with('Z') {
            result.push(cycle_starts_at + i);
        }
    }

    result
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
