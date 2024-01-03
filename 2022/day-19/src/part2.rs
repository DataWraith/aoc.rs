use crate::{part1::solve, structs::*};

pub fn part2(input: &PuzzleInput) -> String {
    input
        .blueprints
        .iter()
        .take(3)
        .map(|bp| solve(bp, 32))
        .product::<isize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part2() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part2(&input), (56 * 62).to_string());
    }
}
