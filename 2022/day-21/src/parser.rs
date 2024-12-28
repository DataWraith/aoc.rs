use utility_belt::prelude::*;

#[derive(Clone, Debug)]
pub enum Operation {
    Plus,
    Minus,
    Times,
    DividedBy,
    Matches,
}

#[derive(Clone, Debug)]
pub enum Monkey {
    Number(i64),
    Operation(&'static str, Operation, &'static str),
}

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub monkeys: HashMap<&'static str, Monkey>,
}

pub fn part1(input: &'static str) -> PuzzleInput {
    let mut monkeys = HashMap::new();

    for line in input.lines() {
        let (name, rest) = line.split_once(": ").unwrap();
        let monkey = parse_monkey(rest);
        monkeys.insert(name, monkey);
    }

    PuzzleInput { monkeys }
}

pub fn part2(input: &'static str) -> PuzzleInput {
    let mut monkeys = HashMap::new();

    for line in input.lines() {
        let (name, rest) = line.split_once(": ").unwrap();
        let monkey = parse_monkey(rest);

        if name == "root" {
            let (left, _op, right) = match &monkey {
                Monkey::Operation(left, _op, right) => (left, Operation::Matches, right),
                _ => panic!("Root is not an operation"),
            };

            monkeys.insert(name, Monkey::Operation(left, Operation::Matches, right));
        } else {
            monkeys.insert(name, monkey);
        }
    }

    PuzzleInput { monkeys }
}

pub fn parse_monkey(input: &'static str) -> Monkey {
    if let Ok(number) = input.parse::<i64>() {
        Monkey::Number(number)
    } else {
        let (left, operator, right) = input.splitn(3, ' ').collect_tuple().unwrap();
        let operation = match operator {
            "+" => Operation::Plus,
            "-" => Operation::Minus,
            "*" => Operation::Times,
            "/" => Operation::DividedBy,
            _ => panic!("Invalid operator: {}", operator),
        };

        Monkey::Operation(left, operation, right)
    }
}
