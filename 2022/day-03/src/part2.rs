use crate::{part1::priority, structs::*};

use utility_belt::prelude::*;

pub fn part2(input: &PuzzleInput) -> String {
    let mut sum = 0;

    for group in input.rucksacks.chunks(3) {
        let itemsets = group
            .iter()
            .map(|r| {
                r.left_compartment
                    .iter()
                    .chain(r.right_compartment.iter())
                    .fold(HashSet::default(), |mut acc, c| {
                        acc.insert(*c);
                        acc
                    })
            })
            .collect::<Vec<HashSet<char>>>();

        let badge_set = itemsets
            .into_iter()
            .reduce(|acc, c| acc.intersection(&c).cloned().collect())
            .unwrap();

        sum += priority(badge_set.into_iter().next().unwrap())
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
