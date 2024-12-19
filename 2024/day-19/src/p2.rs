use cached::proc_macro::cached;

use crate::parser::*;

pub fn part2(input: &PuzzleInput) -> String {
    let mut c = 0;

    for design in input.desired_designs.iter() {
        c += count_possibilities(input, design);
    }

    c.to_string()
}

#[cached(key="String", convert="{design.to_string()}")]
pub fn count_possibilities(
    input: &PuzzleInput,
    design: &str,
) -> usize {
    if design.is_empty() {
        return 1;
    }

    let mut count = 0;

    for prefix_len in (1..=input.longest_pattern).rev() {
        if prefix_len > design.len() {
            continue;
        }

        if input.patterns.contains(&design[..prefix_len]) {
            count += count_possibilities(input, &design[prefix_len..]);
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::indoc;

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
