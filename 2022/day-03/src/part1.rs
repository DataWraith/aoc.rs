use crate::structs::*;

use utility_belt::prelude::*;

pub fn part1(input: &PuzzleInput) -> String {
    input
        .rucksacks
        .iter()
        .map(|r| priority(duplicate_item(r)))
        .sum::<usize>()
        .to_string()
}

pub fn duplicate_item(r: &Rucksack) -> char {
    let left = HashSet::from_iter(r.left_compartment.iter().cloned());
    let right = HashSet::from_iter(r.right_compartment.iter().cloned());

    let mut intersection = left.intersection(&right);
    let item = intersection.next().unwrap();

    assert_eq!(intersection.next(), None);

    *item
}

pub fn priority(c: char) -> usize {
    if c.is_ascii_lowercase() {
        c as usize - 'a' as usize + 1
    } else {
        c as usize - 'A' as usize + 27
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part1(&input), "157");
    }
}
