use utility_belt::prelude::*;

use crate::{p1::hash, parser::*};

pub fn part2(input: &PuzzleInput) -> String {
    let mut potential_earnings = HashMap::new();

    for seed in input.numbers.iter() {
        let prices = prices(*seed);
        let differences = price_differences(&prices);

        let seq_map = sequences_and_earnings(&differences, &prices);

        for (seq, bananas) in seq_map.iter() {
            potential_earnings
                .entry(*seq)
                .and_modify(|b| *b += *bananas as u64)
                .or_insert(*bananas as u64);
        }
    }

    potential_earnings.values().max().unwrap().to_string()
}

// Calculates the potential earnings for each sequence of 4 price differences in
// the data.
pub fn sequences_and_earnings(differences: &[i8], prices: &[u8]) -> HashMap<[i8; 4], u8> {
    differences
        .windows(4)
        // We need to skip the first window, because there is no difference before the first price
        .skip(1)
        .enumerate()
        .fold(HashMap::new(), |mut map, (i, w)| {
            let mut seq = [0; 4];
            seq.copy_from_slice(w);

            if !map.contains_key(&seq) {
                // Since windows() goes forward in time instead of backward, we
                // need to add 4 to the index to get the correct price
                map.insert(seq, prices[i + 4]);
            }

            map
        })
}

// Calculates the prices for a given seed.
pub fn prices(seed: u64) -> Vec<u8> {
    std::iter::successors(Some(seed), |&seed| Some(hash(seed)))
        .take(2000)
        .map(|x| (x % 10) as u8)
        .collect()
}

// Calculates the price differences for a given sequence of prices. Note that
// the difference sequence must start with an arbitrary padding value to line up
// the indices with the prices.
pub fn price_differences(prices: &[u8]) -> Vec<i8> {
    std::iter::once(0)
        .chain(prices.windows(2).map(|w| w[1] as i8 - w[0] as i8))
        .collect()
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
