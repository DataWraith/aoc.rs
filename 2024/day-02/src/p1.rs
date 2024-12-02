use crate::structs::*;

#[tracing::instrument(skip(input))]
pub fn part1(input: &PuzzleInput) -> String {
    input
        .reports
        .iter()
        .filter(|r| is_safe(r))
        .count()
        .to_string()
}

pub fn is_safe(report: &Vec<i64>) -> bool {
    (is_increasing(report) || is_decreasing(report)) && abs_diff_correct(report)
}

fn is_increasing(report: &Vec<i64>) -> bool {
    report.windows(2).all(|w| w[0] < w[1])
}

fn is_decreasing(report: &Vec<i64>) -> bool {
    report.windows(2).all(|w| w[0] > w[1])
}

fn abs_diff_correct(report: &Vec<i64>) -> bool {
    report
        .windows(2)
        .all(|w| w[0].abs_diff(w[1]) >= 1 && w[0].abs_diff(w[1]) <= 3)
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = indoc! {"
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
    "};

    #[test]
    fn test_part1() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part1(&input), "2");
    }
}
