use crate::structs::*;

use utility_belt::prelude::*;

#[tracing::instrument(skip(input))]
pub fn part2(input: &PuzzleInput) -> String {
    let mut right_counts = HashMap::new();

    for right in &input.right {
        *right_counts.entry(right).or_insert(0) += 1;
    }

    let mut sum = 0;

    for left in &input.left {
        if right_counts.contains_key(left) {
            sum += right_counts[left] * left;
        }
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = indoc! {"
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
    "};

    #[test]
    fn test_part2() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part2(&input), "31");
    }
}
