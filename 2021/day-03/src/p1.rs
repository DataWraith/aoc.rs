use crate::parser::*;

pub fn part1(input: &PuzzleInput) -> String {
    let mut gamma = String::new();
    let mut epsilon = String::new();

    for i in 0..input.report[0].len() {
        let mut ones_count = 0;
        let mut zeros_count = 0;

        for j in 0..input.report.len() {
            if input.report[j].chars().nth(i).unwrap() == '1' {
                ones_count += 1;
            } else {
                zeros_count += 1;
            }
        }

        if ones_count > zeros_count {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }

    let gamma = u32::from_str_radix(&gamma, 2).unwrap();
    let epsilon = u32::from_str_radix(&epsilon, 2).unwrap();

    (gamma * epsilon).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = indoc! {"
        00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010
    "};

    #[test]
    fn test_part1_example() {
        let input = crate::parser::part1(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(part1(&input), "198");
    }
}
