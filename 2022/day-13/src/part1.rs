use crate::structs::*;

pub fn part1(input: &PuzzleInput) -> String {
    let mut sum = 0;

    for (i, (a, b)) in input.packets.iter().enumerate() {
        if a < b {
            sum += i + 1;
        }
    }

    sum.to_string()
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
