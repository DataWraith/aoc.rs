use std::collections::BTreeSet;

use utility_belt::prelude::*;

use crate::parser::*;

pub fn part1(input: &PuzzleInput) -> String {
    possible_patterns(input).len().to_string()
}

pub fn possible_patterns(input: &PuzzleInput) -> HashSet<String> {
    let mut q = BTreeSet::new();
    let mut c = HashSet::new();

    for (i, design) in input.desired_designs.iter().enumerate() {
        q.insert((i, design.as_str()));
    }

    while let Some((idx, design)) = q.pop_first() {
        for pattern in input.patterns.iter() {
            if design.starts_with(pattern) {
                if design.len() == pattern.len() {
                    c.insert(input.desired_designs[idx].clone());
                    q.retain(|d| d.0 != idx);
                } else {
                    q.insert((idx, &design[pattern.len()..]));
                }
            }
        }
    }

    c
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = indoc! {"
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
"};

    #[test]
    fn test_part1_example() {
        let input = crate::parser::part1(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(part1(&input), "6");
    }
}
