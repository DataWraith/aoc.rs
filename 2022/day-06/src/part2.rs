use crate::structs::*;

use utility_belt::prelude::*;

pub fn part2(input: &PuzzleInput) -> String {
    (input
        .sequence
        .iter()
        .as_slice()
        .windows(14)
        .enumerate()
        .take_while(|(_, w)| HashSet::from_iter(w.iter().cloned()).len() != 14)
        .last()
        .unwrap()
        .0
        + 14
        + 1)
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part2() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part2(&input), "19");
    }
}
