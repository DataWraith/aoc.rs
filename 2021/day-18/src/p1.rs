use crate::parser::*;

pub fn part1(input: &PuzzleInput) -> String {
    let result = input
        .numbers
        .iter()
        .cloned()
        .reduce(|acc, number| addition(acc, number))
        .unwrap();

    magnitude(result).to_string()
}

pub fn magnitude(input: SnailfishNumber) -> u64 {
    match input {
        SnailfishNumber::Literal(n) => n,
        SnailfishNumber::Pair(left, right) => 3 * magnitude(*left) + 2 * magnitude(*right),
    }
}

pub fn first(input: &mut SnailfishNumber) -> &mut u64 {
    match input {
        SnailfishNumber::Literal(n) => n,
        SnailfishNumber::Pair(left, _) => first(left),
    }
}

pub fn last(input: &mut SnailfishNumber) -> &mut u64 {
    match input {
        SnailfishNumber::Literal(n) => n,
        SnailfishNumber::Pair(_, right) => last(right),
    }
}

pub fn addition(left: SnailfishNumber, right: SnailfishNumber) -> SnailfishNumber {
    let mut result = SnailfishNumber::Pair(Box::new(left), Box::new(right));
    reduce(&mut result);
    result
}

pub fn reduce(input: &mut SnailfishNumber) {
    loop {
        if explode(input, 0, None, None) {
            continue;
        }

        if split(input) {
            continue;
        }

        break;
    }
}

pub fn explode(
    input: &mut SnailfishNumber,
    nesting: u8,
    prev_left: Option<&mut u64>,
    prev_right: Option<&mut u64>,
) -> bool {
    match input {
        SnailfishNumber::Literal(_) => return false,
        SnailfishNumber::Pair(left, right) => {
            if nesting == 4 {
                if let Some(prev_left) = prev_left {
                    *prev_left += *first(left);
                }

                if let Some(prev_right) = prev_right {
                    *prev_right += *first(right);
                }

                *input = SnailfishNumber::Literal(0);

                return true;
            }

            return explode(left, nesting + 1, prev_left, Some(first(right)))
                || explode(right, nesting + 1, Some(last(left)), prev_right);
        }
    }
}

pub fn split(input: &mut SnailfishNumber) -> bool {
    match input {
        SnailfishNumber::Literal(n) => {
            if *n >= 10 {
                let left = SnailfishNumber::Literal(*n / 2);
                let right = SnailfishNumber::Literal((*n + 1) / 2);

                *input = SnailfishNumber::Pair(Box::new(left), Box::new(right));

                return true;
            }

            false
        }

        SnailfishNumber::Pair(left, right) => split(left) || split(right),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;
    use rstest::rstest;

    #[test]
    fn test_part1_example() {
        let input = parser::part1(parser::TEST_INPUT);
        assert_ne!(parser::TEST_INPUT.trim(), "TODO");
        assert_eq!(part1(&input), "4140");
    }

    #[test]
    fn test_addition() {
        let left = parser::part1("[[[[4,3],4],4],[7,[[8,4],9]]]");
        let right = parser::part1("[1,1]");
        let expected = parser::part1("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");

        let result = addition(left.numbers[0].clone(), right.numbers[0].clone());
        assert_eq!(result, expected.numbers[0]);
    }

    #[rstest]
    #[case("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]")]
    #[case("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]")]
    #[case("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]")]
    #[case(
        "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
        "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"
    )]
    #[case("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[7,0]]]]")]
    fn test_explode(#[case] input: &'static str, #[case] expected: &'static str) {
        let mut number = parser::part1(input).clone().numbers[0].clone();
        let result = explode(&mut number, 0, None, None);
        assert_eq!(result, true);
        assert_eq!(number, parser::part1(expected).numbers[0]);
    }
}
