use crate::{part1::part1, structs::*};

pub fn part2(input: &PuzzleInput) -> String {
    let hands = input.hands.iter().map(jacks_to_jokers).collect();
    part1(&PuzzleInput { hands })
}

fn jacks_to_jokers(hand: &Hand) -> Hand {
    let mut hand = hand.clone();

    hand.cards.iter_mut().for_each(|card| {
        if let Card::Jack = card {
            *card = Card::Joker;
        }
    });

    hand
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
    fn test_part2() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part2(&input), "5905");
    }
}
