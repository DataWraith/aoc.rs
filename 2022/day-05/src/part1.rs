use crate::structs::*;

use utility_belt::prelude::*;

pub fn part1(input: &PuzzleInput) -> String {
    let mut crates = input.crates.clone();

    for (n, from, to) in input.instructions.iter() {
        for _ in 0..*n {
            let c = crates[*from].pop().unwrap();
            crates[*to].push(c);
        }
    }

    crates.iter().filter_map(|c| c.last()).join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part1(&input), "CMZ");
    }
}
