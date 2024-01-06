use crate::structs::*;

use utility_belt::prelude::*;

pub fn part1(input: &PuzzleInput) -> String {
    let mut monkeys = input.monkeys.clone();

    keepaway(&mut monkeys, 20, |worry_level| worry_level / 3);

    monkey_business_level(&monkeys)
}

pub fn monkey_business_level(monkeys: &[Monkey]) -> String {
    monkeys
        .iter()
        .sorted_by_key(|m| m.inspections)
        .rev()
        .take(2)
        .map(|m| m.inspections)
        .product::<usize>()
        .to_string()
}

pub fn keepaway(monkeys: &mut [Monkey], rounds: usize, worry_fn: impl Fn(usize) -> usize) {
    for _round in 1..=rounds {
        for i in 0..monkeys.len() {
            for item_idx in 0..monkeys[i].items.len() {
                let item = monkeys[i].items[item_idx];
                let mut worry_level = item;

                match monkeys[i].operation_type {
                    OperationType::Add(j) => worry_level += j,
                    OperationType::Square => worry_level *= worry_level,
                    OperationType::Mul(j) => worry_level *= j,
                }

                worry_level = worry_fn(worry_level);

                let target_monkey = if worry_level % monkeys[i].divisible_by == 0 {
                    monkeys[i].true_monkey
                } else {
                    monkeys[i].false_monkey
                };

                monkeys[target_monkey].items.push(worry_level);
            }

            monkeys[i].inspections += monkeys[i].items.len();
            monkeys[i].items.clear();
        }
    }
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
