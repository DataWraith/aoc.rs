use crate::structs::*;

pub fn part1(input: &PuzzleInput) -> String {
    input
        .rucksacks
        .iter()
        .map(|r| priority(duplicate_item(r)))
        .sum::<usize>()
        .to_string()
}

pub fn duplicate_item(r: &Rucksack) -> char {
    let mut intersection = r.left_compartment.intersection(&r.right_compartment);
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

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part1(&input), "157");
    }
}
