use crate::{part2::visited_tiles, structs::*};

pub fn part1(input: &PuzzleInput) -> String {
    visited_tiles(input, 2).len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part1(&input), "13");
    }
}
