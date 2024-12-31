use crate::parser::*;

pub fn part2(input: &PuzzleInput) -> String {
    let windows = input
        .sonar_readings
        .windows(3)
        .map(|w| w.iter().sum::<i64>());

    windows
        .clone()
        .zip(windows.skip(1))
        .filter(|(a, b)| a < b)
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
    fn test_part2_example() {
        let input = crate::parser::part2(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(part2(&input), "5");
    }
}
