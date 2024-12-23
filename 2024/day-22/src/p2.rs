use utility_belt::prelude::*;

use crate::{p1::hash, parser::*};

pub fn part2(input: &PuzzleInput) -> String {
    let mut potential_earnings = HashMap::new();

    for seed in input.numbers.iter() {
        let prices = prices(*seed);
        let mut seen = HashSet::new();

        prices.tuple_windows().for_each(|(a, b, c, d, e)| {
            let seq = [b - a, c - b, d - c, e - d];

            if !seen.insert(seq) {
                return;
            }

            potential_earnings
                .entry(seq)
                .and_modify(|b| *b += e as u64)
                .or_insert(e as u64);
        });
    }

    potential_earnings.values().max().unwrap().to_string()
}

// Calculates the prices for a given seed.
pub fn prices(seed: u64) -> impl Iterator<Item = i8> {
    std::iter::successors(Some(seed), |&seed| Some(hash(seed)))
        .take(2000)
        .map(|x| (x % 10) as i8)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = utility_belt::prelude::indoc! {"
        1
        2
        3
        2024
    "};

    #[test]
    fn test_part2_example() {
        let input = crate::parser::part2(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(part2(&input), "23");
    }
}
