use crate::structs::*;

use utility_belt::prelude::*;

pub fn part2(input: &PuzzleInput) -> String {
    let mut monkeys = input.monkeys.clone();

    let mut limit = monkeys.iter().fold(1, |acc, b| lcm(acc, b.divisible_by));

    for round in 1..=10000 {
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

                worry_level = worry_level.rem_euclid(limit);

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
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part2() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part2(&input), "2713310158");
    }
}
