use utility_belt::prelude::*;

use crate::parser::*;

#[tracing::instrument(skip(input))]
pub fn part1(input: &PuzzleInput) -> String {
    blink_many(&input.stones, 25).to_string()
}

pub fn blink_many(input: &[u64], count: usize) -> usize {
    let states = Counter::from(input.iter().cloned());

    let counts = (0..count).fold(states, |states, _| {
        state_iteration(&states, |input, _| blink(*input), vec![()])
    });

    counts.count_sum()
}

pub gen fn blink(stone: u64) -> u64 {
    if stone == 0 {
        yield 1;
        return;
    }

    // If the number of digits is even, split the number into two halves
    if stone.ilog10() % 2 == 1 {
        let mut stone_str = stone.to_string();
        let right_str = stone_str.split_off(stone_str.len() / 2);

        yield stone_str.parse::<u64>().unwrap();
        yield right_str.parse::<u64>().unwrap();
        return;
    }

    yield stone * 2024;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blink() {
        let input = crate::parser::part1("0 1 10 99 999");
        assert_eq!(blink_many(&input.stones, 1), 7);
    }

    #[test]
    fn test_blink_2() {
        let input = crate::parser::part1("125 17");
        assert_eq!(blink_many(&input.stones, 6), 22);
    }

    #[test]
    fn test_part1_example1() {
        let input = crate::parser::part1("125 17");
        assert_eq!(part1(&input), "55312");
    }
}
