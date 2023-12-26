use crate::structs::*;

pub fn part1(input: &PuzzleInput) -> String {
    let mut hands = input.hands.clone();

    hands.sort();

    hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + (i + 1) * hand.bid)
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::indoc;

    const TEST_INPUT: &str = indoc! {"
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
    "};

    #[test]
    fn test_part1() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part1(&input), "6440");
    }
}
