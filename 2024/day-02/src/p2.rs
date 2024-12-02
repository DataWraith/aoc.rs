use crate::structs::*;

use utility_belt::prelude::*;

#[tracing::instrument(skip(input))]
pub fn part2(input: &PuzzleInput) -> String {
    let mut safe = 0;

    for report in input.reports.iter() {
        if crate::p1::is_safe(report) {
            safe += 1;
        } else {
            for i in 0..report.len() {
                let mut cloned = report.clone();
                cloned.remove(i);
                if crate::p1::is_safe(&cloned) {
                    safe += 1;
                    break;
                }
            }
        }
    }

    safe.to_string()
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
    fn test_part2() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part2(&input), "4");
    }
}
