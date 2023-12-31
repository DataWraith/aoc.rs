use crate::structs::*;

pub fn part1(input: &PuzzleInput) -> String {
    input
        .assignments
        .iter()
        .filter(|(a, b)| {
            (a.contains(b.start()) && a.contains(b.end()))
                || (b.contains(a.start()) && b.contains(a.end()))
        })
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part1(&input), "2");
    }
}
