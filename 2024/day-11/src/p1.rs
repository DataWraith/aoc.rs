use utility_belt::prelude::*;

use crate::parser::*;

#[tracing::instrument(skip(input))]
pub fn part1(input: &PuzzleInput) -> String {
    blink_many(&input.stones, 25).to_string()
}

pub fn blink_many(input: &[u64], count: usize) -> usize {
    let mut states = HashMap::new();

    for stone in input.iter() {
        states
            .entry(*stone)
            .and_modify(|c| *c += 1)
            .or_insert(1usize);
    }

    for _ in 0..count {
        states = state_iteration(&states, |input, _| blink(input), vec![()]);
    }

    let mut sum = 0;

    for (_, count) in states.iter() {
        sum += count;
    }

    sum
}

pub fn blink(stone: &u64) -> Vec<u64> {
    let mut result = Vec::new();

    if *stone == 0 {
        result.push(1);
        return result;
    }

    if stone.ilog10() % 2 == 1 {
        let mut stone_str = stone.to_string();
        let right_str = stone_str.split_off(stone_str.len() / 2);

        result.push(stone_str.parse::<u64>().unwrap());
        result.push(right_str.parse::<u64>().unwrap());
        return result;
    }

    result.push(*stone * 2024);

    result
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
