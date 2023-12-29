use crate::structs::*;

pub fn part2(input: &PuzzleInput) -> String {
    let mut count = 0;

    let num_matches = input
        .cards
        .iter()
        .map(|card| card.num_matches())
        .collect::<Vec<_>>();

    let mut num_copies = vec![1; input.cards.len()];

    for i in 0..input.cards.len() {
        count += num_copies[i];

        if num_matches[i] == 0 {
            continue;
        }

        for j in (i + 1)..=(i + num_matches[i]) {
            num_copies[j] += num_copies[i];
        }
    }

    count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::indoc;

    const TEST_INPUT: &str = indoc! {"
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    "};

    #[test]
    fn test_part2() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part2(&input), "30");
    }
}
