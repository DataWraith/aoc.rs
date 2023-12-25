use crate::structs::*;

pub fn part1(input: PuzzleInput) -> String {
    let mut sum = 0;

    for line in input.lines.lines() {
        let mut digits = line.chars().filter(|c| c.is_ascii_digit());

        let leading_digit = digits.next().unwrap();
        let trailing_digit = digits.last().unwrap_or(leading_digit);

        let leading = leading_digit.to_digit(10).unwrap();
        let trailing = trailing_digit.to_digit(10).unwrap();

        sum += leading * 10 + trailing;
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = indoc! {"
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
    "};

    #[test]
    fn test_part1() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part1(input), "142");
    }
}
