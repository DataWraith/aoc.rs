use crate::structs::*;

pub fn part1(input: &PuzzleInput) -> String {
    input
        .calories
        .iter()
        .map(|calories| calories.iter().sum::<usize>())
        .max()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part1(&input), "24000");
    }
}
