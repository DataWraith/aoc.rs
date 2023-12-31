use crate::structs::*;

use utility_belt::prelude::*;

pub fn part2(input: &PuzzleInput) -> String {
    let mut crates = input.crates.clone();

    for (n, from, to) in input.instructions.iter() {
        let mut stack = Vec::new();

        for _ in 0..*n {
            stack.push(crates[*from].pop().unwrap());
        }

        while let Some(c) = stack.pop() {
            crates[*to].push(c);
        }
    }

    crates.iter().filter_map(|c| c.last()).join("")
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part2() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part2(&input), "MCD");
    }
}
