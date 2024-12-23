use utility_belt::prelude::*;

use crate::{p1::hash, parser::*};

pub fn part2(input: &PuzzleInput) -> String {
    let mut scores = HashMap::new();

    for seed in input.numbers.iter() {
        let prices = prices(*seed);
        let differences = price_differences(&prices);

        let seq_map = find_sequences(&differences, &prices);

        for (seq, bananas) in seq_map.iter() {
            scores
                .entry(*seq)
                .and_modify(|b| *b += *bananas as u64)
                .or_insert(*bananas as u64);
        }
    }

    scores.values().max().unwrap().to_string()
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

pub fn find_sequences(differences: &[i8], prices: &[u8]) -> HashMap<[i8; 4], u8> {
    differences
        .windows(4)
        .skip(1)
        .enumerate()
        .fold(HashMap::new(), |mut map, (i, w)| {
            let mut seq = [0; 4];
            seq.copy_from_slice(w);

            if !map.contains_key(&seq) {
                map.insert(seq, prices[i + 4]);
            }

            map
        })
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
