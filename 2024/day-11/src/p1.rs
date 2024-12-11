use utility_belt::prelude::*;

use crate::parser::*;

#[tracing::instrument(skip(input))]
pub fn part1(input: &PuzzleInput) -> String {
    blink_many(&input.stones, 25).to_string()
}

pub fn blink_many(input: &String, count: usize) -> usize {
    let mut states = HashMap::new();

    for stone in input.split_ascii_whitespace() {
        states
            .entry(stone.to_string())
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

pub fn blink(stone: &String) -> Vec<String> {
    let mut result = Vec::new();
    let mut stone = stone.clone();

    if stone == "0" {
        result.push(String::from("1"));
    } else if stone.len() % 2 == 0 {
        let right = stone.split_off(stone.len() / 2);
        let right = right.trim_start_matches("0");
        result.push(stone.clone());

        if right.is_empty() {
            result.push(String::from("0"));
        } else {
            result.push(String::from(right));
        }
    } else {
        let num = stone.parse::<u64>().unwrap();
        let num = num * 2024;
        result.push(num.to_string());
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blink() {
        let input = "0 1 10 99 999";
        assert_eq!(blink_many(&mut input.to_string(), 1), 7);
    }

    #[test]
    fn test_blink_2() {
        let input = "125 17";
        assert_eq!(blink_many(&input.to_string(), 6), 22);
    }

    const TEST_INPUT: &str = "125 17";

    #[test]
    fn test_part1_example1() {
        let input = crate::parser::part1(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO");
        assert_eq!(part1(&input), "55312");
    }
}
