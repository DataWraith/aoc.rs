use crate::parser::*;

pub fn part2(input: &PuzzleInput) -> String {
    let oxygen_rating = extract_rating(&input.report, |ones_count, zeros_count| {
        ones_count >= zeros_count
    });

    let co2_rating = extract_rating(&input.report, |ones_count, zeros_count| {
        ones_count < zeros_count
    });

    (oxygen_rating * co2_rating).to_string()
}

fn extract_rating(report: &[&str], keep_ones: impl Fn(usize, usize) -> bool) -> usize {
    let mut report = report.to_vec();

    for i in 0..report[0].len() {
        let mut ones_count = 0;
        let mut zeros_count = 0;

        for bits in report.iter() {
            if bits.chars().nth(i).unwrap() == '1' {
                ones_count += 1;
            } else {
                zeros_count += 1;
            }
        }

        if keep_ones(ones_count, zeros_count) {
            report.retain(|line| line.chars().nth(i).unwrap() == '1');
        } else {
            report.retain(|line| line.chars().nth(i).unwrap() == '0');
        }

        if report.len() == 1 {
            break;
        }
    }

    usize::from_str_radix(&report[0], 2).unwrap()
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
    fn test_part2_example() {
        let input = crate::parser::part2(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(part2(&input), "230");
    }
}
