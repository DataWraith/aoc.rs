use crate::{
    part1::{keepaway, monkey_business_level},
    structs::*,
};

use utility_belt::prelude::*;

pub fn part2(input: &PuzzleInput) -> String {
    let mut monkeys = input.monkeys.clone();

    let limit = monkeys.iter().fold(1, |acc, b| lcm(acc, b.divisible_by));

    keepaway(&mut monkeys, 10000, |worry_level| {
        worry_level.rem_euclid(limit)
    });

    monkey_business_level(&monkeys)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part2() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part2(&input), "2713310158");
    }
}
