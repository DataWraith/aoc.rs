use utility_belt::prelude::*;

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub hands: Vec<Hand>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Hash)]
pub enum Card {
    Ace = 14,
    King = 13,
    Queen = 12,
    Jack = 11,
    Ten = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
    Joker = 1,
}

impl From<char> for Card {
    fn from(c: char) -> Self {
        match c {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'J' => Card::Jack,
            'T' => Card::Ten,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            _ => panic!("Invalid card"),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Hand {
    pub cards: [Card; 5],
    pub bid: usize,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.classify(), self.cards).cmp(&(other.classify(), other.cards))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord)]
pub enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

impl Hand {
    pub fn classify_no_jokers(&self) -> HandType {
        let c = &self.cards;
        let mut s = HashMap::new();

        for card in c.iter() {
            let count = s.entry(card).or_insert(0);
            *count += 1;
        }

        match s.len() {
            // AAAAA
            1 => HandType::FiveOfAKind,

            2 => {
                // AAAAB
                if s.values().any(|&x| x == 4) {
                    return HandType::FourOfAKind;
                }

                // AAABB
                HandType::FullHouse
            }

            3 => {
                // AAABC
                if s.values().any(|&x| x == 3) {
                    return HandType::ThreeOfAKind;
                }

                // AABBC
                HandType::TwoPair
            }

            // AABCD
            4 => HandType::OnePair,
            // ABCDE
            5 => HandType::HighCard,

            _ => unreachable!("Only five cards to a hand."),
        }
    }

    pub fn classify(&self) -> HandType {
        let c = &self.cards;
        let mut s = HashMap::new();

        for card in c.iter() {
            let count = s.entry(card).or_insert(0);
            *count += 1;
        }

        let num_jokers = *s.get(&Card::Joker).unwrap_or(&0);

        if num_jokers == 0 {
            return self.classify_no_jokers();
        }

        match s.len() {
            // AAAAA
            1 => HandType::FiveOfAKind,
            2 => HandType::FiveOfAKind,
            3 => {
                // AAABC
                if s.values().any(|&x| x == 3) {
                    return HandType::FourOfAKind;
                }

                // AABBC
                if num_jokers == 1 {
                    return HandType::FullHouse;
                }

                HandType::FourOfAKind
            }
            4 => {
                // AABCD
                if s.values().filter(|&x| *x == 2).count() == 1 {
                    if num_jokers == 1 || num_jokers == 2 {
                        return HandType::ThreeOfAKind;
                    }

                    return HandType::TwoPair;
                }

                // AABBC
                if num_jokers == 1 {
                    return HandType::FullHouse;
                }

                HandType::FourOfAKind
            }
            5 => HandType::OnePair,
            _ => unreachable!("Only five cards to a hand."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classify_test() {
        let hand = Hand {
            cards: [Card::Ace, Card::Ace, Card::Ace, Card::Ace, Card::Ace],
            bid: 1,
        };

        assert_eq!(hand.classify_no_jokers(), HandType::FiveOfAKind);

        let hand = Hand {
            cards: [Card::Ace, Card::Ace, Card::Ace, Card::Ace, Card::King],
            bid: 1,
        };

        assert_eq!(hand.classify_no_jokers(), HandType::FourOfAKind);

        let hand = Hand {
            cards: [Card::Ace, Card::Ace, Card::Ace, Card::King, Card::King],
            bid: 1,
        };

        assert_eq!(hand.classify_no_jokers(), HandType::FullHouse);

        let hand = Hand {
            cards: [Card::Ace, Card::Ace, Card::Ace, Card::King, Card::Queen],
            bid: 1,
        };

        assert_eq!(hand.classify_no_jokers(), HandType::ThreeOfAKind);

        let hand = Hand {
            cards: [Card::Ace, Card::Ace, Card::King, Card::King, Card::Queen],
            bid: 1,
        };

        assert_eq!(hand.classify_no_jokers(), HandType::TwoPair);

        let hand = Hand {
            cards: [Card::Ace, Card::Ace, Card::King, Card::Queen, Card::Jack],
            bid: 1,
        };

        assert_eq!(hand.classify_no_jokers(), HandType::OnePair);

        let hand = Hand {
            cards: [Card::Ace, Card::King, Card::Queen, Card::Jack, Card::Ten],
            bid: 1,
        };

        assert_eq!(hand.classify_no_jokers(), HandType::HighCard);
    }

    #[test]
    fn test_hand_ordering() {
        let hand1 = Hand {
            cards: [
                Card::Three,
                Card::Three,
                Card::Three,
                Card::Three,
                Card::Two,
            ],
            bid: 1,
        };

        let hand2 = Hand {
            cards: [Card::Two, Card::Ace, Card::Ace, Card::Ace, Card::Ace],
            bid: 1,
        };

        assert!(hand1 > hand2);

        let hand1 = Hand {
            cards: [
                Card::Seven,
                Card::Seven,
                Card::Eight,
                Card::Eight,
                Card::Eight,
            ],
            bid: 1,
        };

        let hand2 = Hand {
            cards: [
                Card::Seven,
                Card::Seven,
                Card::Seven,
                Card::Eight,
                Card::Eight,
            ],
            bid: 1,
        };

        assert!(hand1 > hand2);
    }

    #[test]
    fn test_hand_ordering2() {
        let rank1 = Hand {
            cards: [Card::Three, Card::Two, Card::Ten, Card::Three, Card::King],
            bid: 1,
        };

        assert_eq!(rank1.classify(), HandType::OnePair);

        let rank2 = Hand {
            cards: [Card::King, Card::King, Card::Six, Card::Seven, Card::Seven],
            bid: 2,
        };

        assert_eq!(rank2.classify(), HandType::TwoPair);

        let rank3 = Hand {
            cards: [Card::Ten, Card::Five, Card::Five, Card::Joker, Card::Five],
            bid: 3,
        };

        assert_eq!(rank3.classify(), HandType::FourOfAKind);

        let rank4 = Hand {
            cards: [
                Card::Queen,
                Card::Queen,
                Card::Queen,
                Card::Joker,
                Card::Ace,
            ],
            bid: 4,
        };

        assert_eq!(rank4.classify(), HandType::FourOfAKind);

        let rank5 = Hand {
            cards: [Card::King, Card::Ten, Card::Joker, Card::Joker, Card::Ten],
            bid: 5,
        };

        assert_eq!(rank5.classify(), HandType::FourOfAKind);

        let mut hands = vec![
            rank1.clone(),
            rank2.clone(),
            rank3.clone(),
            rank4.clone(),
            rank5.clone(),
        ];

        hands.sort();

        assert_eq!(hands, vec![rank1, rank2, rank3, rank4, rank5]);
    }

    #[test]
    fn test_classify_part2() {
        let hand = Hand {
            cards: [Card::Ace, Card::Ace, Card::Ace, Card::Ace, Card::Ace],
            bid: 1,
        };

        assert_eq!(hand.classify(), HandType::FiveOfAKind);

        let hand = Hand {
            cards: [Card::Ace, Card::Ace, Card::Ace, Card::Ace, Card::King],
            bid: 1,
        };

        assert_eq!(hand.classify(), HandType::FourOfAKind);

        let hand = Hand {
            cards: [Card::Ace, Card::Ace, Card::Ace, Card::King, Card::King],
            bid: 1,
        };

        assert_eq!(hand.classify(), HandType::FullHouse);

        let hand = Hand {
            cards: [Card::Ace, Card::Ace, Card::Ace, Card::King, Card::Queen],
            bid: 1,
        };

        assert_eq!(hand.classify(), HandType::ThreeOfAKind);

        let hand = Hand {
            cards: [Card::Ace, Card::Ace, Card::King, Card::King, Card::Queen],
            bid: 1,
        };

        assert_eq!(hand.classify(), HandType::TwoPair);

        let hand = Hand {
            cards: [Card::Ace, Card::Ace, Card::King, Card::Queen, Card::Joker],
            bid: 1,
        };

        assert_eq!(hand.classify(), HandType::ThreeOfAKind);

        let hand = Hand {
            cards: [Card::Ace, Card::King, Card::Queen, Card::Joker, Card::Ten],
            bid: 1,
        };

        assert_eq!(hand.classify(), HandType::OnePair);

        let hand = Hand {
            cards: [Card::Ace, Card::Eight, Card::Joker, Card::Five, Card::Joker],
            bid: 1,
        };

        assert_eq!(hand.classify(), HandType::ThreeOfAKind);
    }
}
