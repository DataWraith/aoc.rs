use crate::{p1::lanternfish_after_days, parser::*};

pub fn part2(input: &PuzzleInput) -> String {
    lanternfish_after_days(&input.lanternfish, 256).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "3,4,3,1,2";

    #[test]
    fn test_part2_example() {
        let input = crate::parser::part2(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(part2(&input), "26984457539");
    }
}
