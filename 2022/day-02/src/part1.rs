use crate::structs::*;

pub fn part1(input: &PuzzleInput) -> String {
    input
        .guide
        .iter()
        .map(|guide| guide.1.part1_score(guide.0))
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part1(&input), "15");
    }
}
