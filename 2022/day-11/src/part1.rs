use crate::structs::*;

use utility_belt::prelude::*;

pub fn part1(input: &PuzzleInput) -> String {
    let mut monkeys = input.monkeys.clone();

    for _round in 1..=20 {
        for i in 0..monkeys.len() {
            while !monkeys[i].items.is_empty() {
                let item = monkeys[i].items.remove(0);
                let mut worry_level = item;

                match monkeys[i].operation_type {
                    OperationType::Add(j) => worry_level += j,
                    OperationType::Square => worry_level *= worry_level,
                    OperationType::Mul(j) => worry_level *= j,
                }

                monkeys[i].inspections += 1;

                worry_level /= 3;

                let target_monkey = if worry_level % monkeys[i].divisible_by == 0 {
                    monkeys[i].true_monkey
                } else {
                    monkeys[i].false_monkey
                };

                monkeys[target_monkey].items.push(worry_level);
            }
        }
    }

    monkeys.sort_by_key(|m| m.inspections);

    let m1 = monkeys.pop().unwrap();
    let m2 = monkeys.pop().unwrap();

    (m1.inspections * m2.inspections).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part1(&input), "10605");
    }
}
