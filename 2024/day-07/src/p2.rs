use crate::structs::*;

#[tracing::instrument(skip(input))]
pub fn part2(input: &PuzzleInput) -> String {
    input
        .equations
        .iter()
        .filter(|(target, numbers)| can_be_solved(*target, numbers))
        .map(|(target, _)| *target)
        .sum::<i64>()
        .to_string()
}

fn can_be_solved(target: i64, remainder: &[i64]) -> bool {
    if remainder.len() == 1 {
        return remainder[0] == target;
    }

    let next = remainder.last().unwrap();
    let next_remainder = &remainder[..remainder.len() - 1];

    if target > *next && can_be_solved(target - next, next_remainder) {
        return true;
    }

    if target % next == 0 && can_be_solved(target / next, next_remainder) {
        return true;
    }

    let left = target.to_string();
    let right = next.to_string();

    if left.len() > right.len() && left.ends_with(&right) {
        let new_target = left[..(left.len() - right.len())].parse::<i64>().unwrap();
        return can_be_solved(new_target, next_remainder);
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = utility_belt::prelude::indoc! {"
        190: 10 19
        3267: 81 40 27
        83: 17 5
        156: 15 6
        7290: 6 8 6 15
        161011: 16 10 13
        192: 17 8 14
        21037: 9 7 18 13
        292: 11 6 16 20
    "};

    #[test]
    fn test_part2_example() {
        let input = crate::parser::part2(TEST_INPUT);
        assert_eq!(part2(&input), "11387");
    }
}
