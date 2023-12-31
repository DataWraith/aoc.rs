use crate::structs::*;

use utility_belt::prelude::*;

pub fn part2(input: &PuzzleInput) -> String {
    input
        .assignments
        .iter()
        .filter(|(a, b)| {
            (a.contains(b.start()) || a.contains(b.end()))
                || (b.contains(a.start()) || b.contains(a.end()))
        })
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part2() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part2(&input), "4");
    }
}
