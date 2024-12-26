use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline},
    combinator::{eof, opt},
    multi::{many1, separated_list1},
    IResult, Parser,
};

use crate::structs::*;

fn nom_parser(input: &str) -> IResult<&str, PuzzleInput> {
    let (input, monkeys) = many1(parse_monkey)(input)?;
    let (input, _) = eof(input)?;

    Ok((input, PuzzleInput { monkeys }))
}

pub fn parse(input: &str) -> PuzzleInput {
    nom_parser(input).unwrap().1
}

pub fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _) = tag("Monkey ")(input)?;
    let (input, _) = digit1(input)?;
    let (input, _) = tag(":\n  Starting items: ")(input)?;
    let (input, first_item) = digit1(input)?;
    let (input, _) = opt(tag(", "))(input)?;
    let (input, items) = opt(separated_list1(tag(", "), digit1))(input)?;
    let (input, _) = tag("\n  Operation: new = ")(input)?;
    let (input, operation_type) = parse_operation(input)?;
    let (input, _) = tag("\n  Test: divisible by ")(input)?;
    let (input, divisible_by) = digit1(input)?;
    let (input, _) = tag("\n    If true: throw to monkey ")(input)?;
    let (input, true_monkey) = digit1(input)?;
    let (input, _) = tag("\n    If false: throw to monkey ")(input)?;
    let (input, false_monkey) = digit1(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = opt(newline)(input)?;

    Ok((
        input,
        Monkey {
            operation_type,
            items: std::iter::once(vec![first_item])
                .chain(items)
                .flat_map(move |s| s.into_iter().map(|s| s.parse().unwrap()))
                .collect(),
            divisible_by: divisible_by.parse().unwrap(),
            true_monkey: true_monkey.parse().unwrap(),
            false_monkey: false_monkey.parse().unwrap(),
            inspections: 0,
        },
    ))
}

pub fn parse_operation(input: &str) -> IResult<&str, OperationType> {
    let (input, square) = opt(tag("old * old").map(|_| OperationType::Square))(input)?;
    let (input, addition) = opt(parse_addition)(input)?;
    let (input, multiplication) = opt(parse_multiplication)(input)?;

    Ok((input, square.or(addition).or(multiplication).unwrap()))
}

pub fn parse_addition(input: &str) -> IResult<&str, OperationType> {
    let (input, _) = tag("old + ")(input)?;
    let (input, operand) = digit1(input)?;

    Ok((input, OperationType::Add(operand.parse().unwrap())))
}

pub fn parse_multiplication(input: &str) -> IResult<&str, OperationType> {
    let (input, _) = tag("old * ")(input)?;
    let (input, operand) = digit1(input)?;

    Ok((input, OperationType::Mul(operand.parse().unwrap())))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_parse() {
        assert!(nom_parser(TEST_INPUT).is_ok());
    }
}
