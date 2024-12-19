use std::collections::BTreeSet;

use utility_belt::prelude::*;

use crate::parser::*;

pub fn part1(input: &PuzzleInput) -> String {
    assemble_pattern(input).to_string()
}

pub fn assemble_pattern(input: &PuzzleInput) -> usize {
    let mut q = BTreeSet::new();
    let mut c = HashSet::new();

    for (i, design) in input.desired_designs.iter().enumerate() {
        q.insert((i, design.as_str()));
    }

    while let Some(design) = q.pop_first() {
        for pattern in input.patterns.iter() {
            if design.1.starts_with(pattern) {
                if design.1.len() == pattern.len() {
                    c.insert(design.0);
                    q.retain(|d| d.0 != design.0);
                } else {
                    q.insert((design.0, &design.1[pattern.len()..]));
                }
            }

            if design.1.ends_with(pattern) {
                if design.1.len() == pattern.len() {
                    c.insert(design.0);
                    q.retain(|d| d.0 != design.0);
                } else {
                    q.insert((design.0, &design.1[..design.1.len() - pattern.len()]));
                }
            }
        }
    }

    c.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

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
