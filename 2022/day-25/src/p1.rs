use crate::parser::*;

pub fn part1(input: &PuzzleInput) -> String {
    to_snafu(input.numbers.iter().sum::<i64>())
}

fn to_snafu(mut n: i64) -> String {
    let mut result = String::new();

    while n > 0 {
        let digit = n % 5;

        match digit {
            // 0, 1, 2 are normal digits
            0 => result.push('0'),
            1 => result.push('1'),
            2 => result.push('2'),

            // 3, 4 are negative digits. They essentially borrow from the
            // remainder of the number, so we can make things work out by just
            // adding that back.
            3 => {
                result.push('=');
                n += 2;
            }
            4 => {
                result.push('-');
                n += 1;
            }
            _ => panic!("Invalid digit: {}", digit),
        }

        n /= 5;
    }

    result.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = indoc! {"
        1=-0-2
        12111
        2=0=
        21
        2=01
        111
        20012
        112
        1=-1=
        1-12
        12
        1=
        122
    "};

    #[test]
    fn test_part1_example() {
        let input = crate::parser::part1(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(part1(&input), "2=-1=0");
    }
}
