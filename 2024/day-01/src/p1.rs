use crate::structs::*;

use utility_belt::prelude::*;

#[tracing::instrument]
pub fn part1(input: &PuzzleInput) -> String {
    let mut left = input.left.clone();
    let mut right = input.right.clone();

    left.sort();
    right.sort();

    let mut differences = Vec::new();

    for i in 0..left.len() {
        differences.push(right[i].abs_diff(left[i]));
    }

    return differences.iter().sum::<u32>().to_string();
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
    fn test_part1() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part1(&input), "TODO");
    }
}
