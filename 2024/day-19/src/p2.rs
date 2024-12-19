use std::collections::BTreeMap;

use crate::{p1::possible_patterns, parser::*};

pub fn part2(input: &PuzzleInput) -> String {
    let possible = possible_patterns(input);

    let mut c = 0;

    for design in possible.iter() {
        c += count_possibilities(input, design);
    }

    c.to_string()
}

pub fn count_possibilities(input: &PuzzleInput, design: &str) -> usize {
    let mut c = 0;

    let mut q = BTreeMap::new();
    q.insert(design, 1);

    while let Some((design, paths)) = q.pop_first() {
        if design.is_empty() {
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
