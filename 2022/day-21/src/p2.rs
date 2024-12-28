use crate::{p1::evaluate, parser::*};

pub fn part2(input: &PuzzleInput) -> String {
    find_human_value(input, "root", 1).unwrap().to_string()
}

pub fn find_human_value(input: &PuzzleInput, current: &str, target: i64) -> Option<i64> {
    if current == "humn" {
        return Some(target);
    }

    let (left, op, right) = match &input.monkeys[&current] {
        Monkey::Number(_) => {
            // We can't change the value of a number
            return None;
        }

        Monkey::Operation(left, op, right) => (left, op, right),
    };

    let left_value = evaluate(left, &input.monkeys);
    let right_value = evaluate(right, &input.monkeys);

    // Now we need to find the target value for the left and right branches of
    // the operation. This is possible, because there is only one value we can
    // change (humn), so either the left OR right branch will have to change,
    // but not both.
    let (left_tgt, right_tgt) = match op {
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
            let diff = left_value - right_value;
            (left_value - diff, right_value + diff)
        }
    };

    // Recurse
    find_human_value(input, left, left_tgt).or(find_human_value(input, right, right_tgt))
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
