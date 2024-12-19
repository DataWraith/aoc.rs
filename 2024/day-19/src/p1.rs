use utility_belt::prelude::*;

use crate::{p2::count_possibilities, parser::*};

pub fn part1(input: &PuzzleInput) -> String {
    let mut cache = HashMap::new();
    let mut c = 0;

    for design in input.desired_designs.iter() {
        c += count_possibilities(input, design, &mut cache).min(1);
    }

    c.to_string()
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
