use crate::parser::*;

pub fn part1(input: &PuzzleInput) -> String {
    input
        .sonar_readings
        .windows(2)
        .filter(|w| w[0] < w[1])
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = indoc! {"
        199
        200
        208
        210
        200
        207
        240
        269
        260
        263
    "};

    #[test]
    fn test_part1_example() {
        let input = crate::parser::part1(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(part1(&input), "7");
    }
}
