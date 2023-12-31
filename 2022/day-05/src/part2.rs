use crate::structs::*;

use utility_belt::prelude::*;

pub fn part2(input: &PuzzleInput) -> String {
    let mut crates = input.crates.clone();

    for (n, from, to) in input.instructions.iter() {
        let stack_size = crates[*from].len();
        let claw = crates[*from].split_off(stack_size - n);
        crates[*to].extend(claw);
    }

    crates.iter().filter_map(|c| c.last()).join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part2() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part2(&input), "MCD");
    }
}
