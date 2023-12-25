use crate::structs::*;

pub fn part2(input: PuzzleInput) -> String {
    let translation = [
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    let mut sum = 0;

    for line in input.lines.lines() {
        let mut first_digit = None;
        let mut last_digit = None;

        for idx in 0..line.len() {
            for (word, digit) in translation.iter() {
                if line[idx..].starts_with(word) {
                    first_digit.get_or_insert(*digit);
                    last_digit = Some(*digit);
                }
            }
        }

        sum += first_digit.unwrap() * 10 + last_digit.unwrap();
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = indoc! {"
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
    "};

    #[test]
    fn test_part2() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part2(input), "281");
    }
}
