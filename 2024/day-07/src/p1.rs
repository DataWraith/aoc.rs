use crate::structs::*;

use utility_belt::prelude::*;

#[tracing::instrument(skip(input))]
pub fn part1(input: &PuzzleInput) -> String {
    input
        .equations
        .iter()
        .map(|(target, numbers)| {
            let mut n: Vec<i128> = numbers.iter().map(|n| *n as i128).collect();
            let first = n.remove(0);

            let soln = solve_equation(first, *target as i128, n);

            dbg!(&soln);

            if soln.is_some() {
                *target
            } else {
                0
            }
        })
        .sum::<i64>()
        .to_string()
}

#[memoize::memoize]
fn solve_equation(current: i128, target: i128, remainder: Vec<i128>) -> Option<Vec<char>> {
    if current == target && remainder.is_empty() {
        return Some(vec![]);
    }

    if remainder.is_empty() {
        return None;
    }

    let next = remainder.first().unwrap();
    let next_remainder = remainder[1..].to_vec();

    if let Some(mut solution) = solve_equation(current + next, target, next_remainder.clone()) {
        solution.push('+');
        solution.reverse();
        return Some(solution);
    }

    if let Some(mut solution) = solve_equation(current * next, target, next_remainder) {
        solution.push('*');
        solution.reverse();
        return Some(solution);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

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
    fn test_part1_example() {
        let input = crate::parser::part1(TEST_INPUT);
        dbg!(&input);
        assert_eq!(part1(&input), "3749");
    }
}
