use std::collections::{BTreeMap, BTreeSet};

use utility_belt::prelude::*;

use crate::parser::*;

pub fn part2(input: &PuzzleInput) -> String {
    let possible = possible_patterns(input);

    let mut c = 0;

    for (i, design) in possible.iter().enumerate() {
        dbg!(i, design);
        c += count_possibilities(input, design);
    }

    c.to_string()
}

pub fn count_possibilities(input: &PuzzleInput, design: &str) -> usize {
    let mut c = 0;

    let mut q = BTreeMap::new();
    q.insert(design, 1);

    while let Some((design, paths)) = q.pop_first() {
        if design.len() == 0 {
            c += paths;
            continue;
        }

        for pattern in input.patterns.iter() {
            if design.starts_with(pattern) {
                q.entry(&design[pattern.len()..])
                    .and_modify(|p| *p += paths)
                    .or_insert(paths);
            }
        }
    }

    c
}

pub fn possible_patterns(input: &PuzzleInput) -> HashSet<String> {
    let mut q = BTreeSet::new();
    let mut c = HashSet::new();

    for (i, design) in input.desired_designs.iter().enumerate() {
        q.insert((i, design.as_str()));
    }

    while let Some(design) = q.pop_first() {
        for pattern in input.patterns.iter() {
            if design.1.starts_with(pattern) {
                if design.1.len() == pattern.len() {
                    c.insert(input.desired_designs[design.0].clone());
                    q.retain(|d| d.0 != design.0);
                } else {
                    q.insert((design.0, &design.1[pattern.len()..]));
                }
            }

            if design.1.ends_with(pattern) {
                if design.1.len() == pattern.len() {
                    c.insert(input.desired_designs[design.0].clone());
                    q.retain(|d| d.0 != design.0);
                } else {
                    q.insert((design.0, &design.1[..design.1.len() - pattern.len()]));
                }
            }
        }
    }

    c
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
    fn test_part2_example() {
        let input = crate::parser::part2(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(part2(&input), "16");
    }
}
