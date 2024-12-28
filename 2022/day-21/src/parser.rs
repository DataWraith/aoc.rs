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
    Operation(String, Operation, String),
}

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub monkeys: HashMap<String, Monkey>,
}

pub fn part1(input: &'static str) -> PuzzleInput {
    let mut monkeys = HashMap::new();

    for line in input.lines() {
        let (name, rest) = line.split_once(": ").unwrap();
        let monkey = parse_monkey(rest);
        monkeys.insert(name.to_string(), monkey);
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

            monkeys.insert(
                name.to_string(),
                Monkey::Operation(left.to_string(), Operation::Matches, right.to_string()),
            );
        } else {
            monkeys.insert(name.to_string(), monkey);
        }
    }

    PuzzleInput { monkeys }
}

pub fn parse_monkey(input: &str) -> Monkey {
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

        Monkey::Operation(left.to_string(), operation, right.to_string())
    }
}
