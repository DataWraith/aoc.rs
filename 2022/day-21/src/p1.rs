use utility_belt::prelude::*;

use crate::parser::*;

pub fn part1(input: &PuzzleInput) -> String {
    evaluate(&input.monkeys["root"], &input.monkeys).to_string()
}

pub fn evaluate(monkey: &Monkey, monkeys: &HashMap<String, Monkey>) -> i64 {
    match monkey {
        Monkey::Number(n) => *n,
        Monkey::Operation(left, op, right) => {
            let left_value = evaluate(&monkeys[left], monkeys);
            let right_value = evaluate(&monkeys[right], monkeys);
            match op {
                Operation::Plus => left_value + right_value,
                Operation::Minus => left_value - right_value,
                Operation::Times => left_value * right_value,
                Operation::DividedBy => left_value / right_value,
                Operation::Matches => {
                    if left_value == right_value {
                        1
                    } else {
                        0
                    }
                }
            }
        }
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
    fn test_part1_example() {
        let input = crate::parser::part1(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(part1(&input), "152");
    }
}
