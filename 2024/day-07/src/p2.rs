use crate::structs::*;

use utility_belt::prelude::*;

#[tracing::instrument(skip(input))]
pub fn part2(input: &PuzzleInput) -> String {
    input
        .equations
        .iter()
        .map(|(target, numbers)| {
            let soln = solve_equation(0, *target, numbers);

            if soln {
                *target
            } else {
                0
            }
        })
        .sum::<i64>()
        .to_string()
}

fn solve_equation(current: i64, target: i64, remainder: &[i64]) -> bool {
    if current == target && remainder.is_empty() {
        return true;
    }

    if remainder.is_empty() || current > target {
        return false;
    }

    let next = remainder[0];
    let next_remainder = &remainder[1..];

    if solve_equation(current + next, target, next_remainder)
        || solve_equation(current * next, target, next_remainder)
    {
        return true;
    }

    let left = current.to_string();
    let right = next.to_string();
    let concated = left + &right;
    let concated_num = concated.parse::<i64>().unwrap();

    solve_equation(concated_num, target, next_remainder)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = indoc! {"
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
