use crate::{part1::priority, structs::*};

use utility_belt::prelude::*;

pub fn part2(input: &PuzzleInput) -> String {
    let mut sum = 0;

    for group in input.rucksacks.chunks(3) {
        let badge = group
            .iter()
            .map(|r| {
                r.left_compartment
                    .union(&r.right_compartment)
                    .cloned()
                    .collect::<HashSet<char>>()
            })
            .reduce(|a, b| a.intersection(&b).cloned().collect())
            .unwrap()
            .into_iter()
            .next()
            .unwrap();

        sum += priority(badge)
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part2() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part2(&input), "70");
    }
}
