#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub cards: Vec<Card>,
}

#[derive(Clone, Debug)]
pub struct Card {
    winning_numbers: Vec<usize>,
    numbers_you_have: Vec<usize>,
}

impl Card {
    pub fn num_matches(&self) -> usize {
        let mut count = 0;

        for number in self.numbers_you_have.iter() {
            if self.winning_numbers.contains(number) {
                count += 1;
            }
        }

        count
    }

    pub fn score(&self) -> usize {
        let count = self.num_matches();

        if count == 0 {
            return 0;
        }

        2usize.pow((count - 1) as u32)
    }
}

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        let split = value.split(": ").collect::<Vec<_>>();
        let numbers = split[1].split(" | ").collect::<Vec<_>>();

        let winning_numbers = numbers[0]
            .trim()
            .split(' ')
            .filter(|n| !n.is_empty())
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let numbers_you_have = numbers[1]
            .trim()
            .split(' ')
            .filter(|n| !n.is_empty())
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        Card {
            winning_numbers,
            numbers_you_have,
        }
    }
}
