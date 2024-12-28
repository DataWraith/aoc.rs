use utility_belt::prelude::*;

use crate::{p1::evaluate, parser::*};

pub fn part2(input: &PuzzleInput) -> String {
    let (change_root, needed) = find_branch_to_change(input);

    find_human_value(input, change_root, needed)
        .unwrap()
        .to_string()
}

pub fn find_human_value(input: &PuzzleInput, current: String, target: i64) -> Option<i64> {
    if current == "humn" {
        return Some(target);
    }

    let (left, right) = match &input.monkeys[&current] {
        Monkey::Number(_) => return None,
        Monkey::Operation(left, _op, right) => (left.clone(), right.clone()),
    };

    let left_value = evaluate(&input.monkeys[&left], &input.monkeys);
    let right_value = evaluate(&input.monkeys[&right], &input.monkeys);

    let (left_tgt, right_tgt) = match &input.monkeys[&current] {
        Monkey::Number(_) => return None,

        Monkey::Operation(left, op, right) => match op {
            Operation::Plus => {
                let sum = left_value + right_value;
                let diff = target - sum;

                (left_value + diff, right_value + diff)
            }

            Operation::Minus => {
                let difference = left_value - right_value;
                let diff = target - difference;

                (left_value + diff, right_value - diff)
            }

            Operation::Times => {
                let product = left_value * right_value;
                let diff = target - product;

                (
                    left_value + diff / right_value,
                    right_value + diff / left_value,
                )
            }

            Operation::DividedBy => {
                let quotient = left_value / right_value;
                let diff = target - quotient;

                (
                    left_value + diff * right_value,
                    right_value + diff * left_value,
                )
            }

            Operation::Matches => {
                return None;
            }
        },
    };

    let left = find_human_value(input, left, left_tgt);

    if left.is_some() {
        return left;
    }

    find_human_value(input, right, right_tgt)
}

fn find_branch_to_change(input: &PuzzleInput) -> (String, i64) {
    let root = input.monkeys["root"].clone();

    let (left, right) = match root {
        Monkey::Operation(left, _op, right) => (left, right),
        _ => panic!("Root is not an operation"),
    };

    let eval_orig_left = evaluate(&input.monkeys[&left], &input.monkeys);
    let eval_orig_right = evaluate(&input.monkeys[&right], &input.monkeys);

    let mut changed = input.clone();

    changed
        .monkeys
        .insert("humn".to_string(), Monkey::Number(123));

    let eval_changed_left = evaluate(&changed.monkeys[&left], &changed.monkeys);

    if eval_orig_left == eval_changed_left {
        (right.to_string(), eval_orig_left)
    } else {
        (left.to_string(), eval_orig_right)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = indoc! {"
        root: pppw + sjmn
        dbpl: 5
        cczh: sllz + lgvd
        zczc: 2
        ptdq: humn - dvpt
        dvpt: 3
        lfqf: 4
        humn: 5
        ljgn: 2
        sjmn: drzm * dbpl
        sllz: 4
        pppw: cczh / lfqf
        lgvd: ljgn * ptdq
        drzm: hmdt - zczc
        hmdt: 32
    "};

    #[test]
    fn test_part2_example() {
        let input = crate::parser::part2(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(part2(&input), "301");
    }
}
