use rayon::prelude::*;

use utility_belt::prelude::*;

use crate::{p1::hash, parser::*};

pub fn part2(input: &PuzzleInput) -> String {
    let mut sequences = HashSet::new();

    let mut prices_vec = Vec::new();
    let mut differences_vec = Vec::new();

    for seed in input.numbers.iter() {
        let prices = prices(*seed);
        let differences = price_differences(&prices);

        sequences.extend(find_sequences(&differences));

        prices_vec.push(prices);
        differences_vec.push(differences);
    }

    let mut best_earnings = 0;

    for (i, sequence) in sequences.iter().enumerate() {
        dbg!(i, sequences.len());

        let mut bananas = 0;

        for (j, seed) in input.numbers.iter().enumerate() {
            bananas += earnings(*sequence, &prices_vec[j], &differences_vec[j]);
        }

        if bananas > best_earnings {
            best_earnings = bananas;
        }
    }

    best_earnings.to_string()
}

pub fn earnings(sequence: [i8; 4], prices: &[u8], differences: &[i8]) -> u64 {
    let windows = differences.windows(4).skip(1);

    for (i, window) in windows.enumerate() {
        if window == sequence {
            return prices[i + 4] as u64;
        }
    }

    0
}

pub fn find_sequences(differences: &[i8]) -> HashSet<[i8; 4]> {
    differences
        .windows(4)
        .map(|w| {
            let mut seq = [0; 4];
            seq.copy_from_slice(w);
            seq
        })
        .collect()
}

pub fn prices(seed: u64) -> Vec<u8> {
    let mut prices = Vec::new();
    let mut seed = seed;

    while prices.len() < 2000 {
        prices.push((seed % 10) as u8);
        seed = hash(seed);
    }

    prices
}

pub fn price_differences(prices: &[u8]) -> Vec<i8> {
    let mut differences = vec![0];

    for i in 1..prices.len() {
        differences.push(prices[i] as i8 - prices[i - 1] as i8);
    }

    differences
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = indoc! {"
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
