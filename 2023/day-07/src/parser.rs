use nom::{
    character::complete::{newline, space1},
    combinator::eof,
    IResult,
};

use utility_belt::prelude::*;

use crate::structs::*;

fn nom_parser(input: &str) -> IResult<&str, PuzzleInput> {
    let (input, hands) = nom::multi::many1(parse_line)(input)?;
    let (input, _) = eof(input)?;

    Ok((input, PuzzleInput { hands }))
}

fn parse_line(input: &str) -> IResult<&str, Hand> {
    let (input, cards) = nom::multi::count(parse_card, 5)(input)?;
    let (input, _) = space1(input)?;
    let (input, bid) = parse_usize(input)?;
    let (input, _) = newline(input)?;

    let cards = cards.clone();

    Ok((
        input,
        Hand {
            cards: [cards[0], cards[1], cards[2], cards[3], cards[4]],
            bid,
        },
    ))
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    let (input, card) = nom::character::complete::one_of("AKQJT98765432")(input)?;
    let card = Card::from(card);

    Ok((input, card))
}

pub fn parse(input: &str) -> PuzzleInput {
    nom_parser(input).unwrap().1
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
    fn test_parse() {
        assert!(nom_parser(TEST_INPUT).is_ok());
    }
}
