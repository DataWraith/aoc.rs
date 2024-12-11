use std::collections::LinkedList;

use utility_belt::prelude::*;

use crate::parser::*;

#[tracing::instrument(skip(input))]
pub fn part1(input: &PuzzleInput) -> String {
    let mut stones = input.stones.iter().join(" ");

    blink_many(&mut stones, 25).to_string()
}

pub fn blink_many(input: &mut String, count: usize) -> usize {
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

    /*
    let mut cache: HashMap<String, usize> = HashMap::new();

    let mut result = 0;

    for stone in input.iter() {
        if let Some(cached) = cache.get(stone) {
            result += *cached;
            continue;
        }

        let mut s = LinkedList::new();
        s.push_back(stone.clone());

        for _ in 0..count {
            blink(&mut s);

            let mut c = s.cursor_front_mut();

            while let Some(cur_stone) = c.current() {
                if let Some(cached) = cache.get(cur_stone) {
                    result += *cached;
                    c.remove_current();
                }
            }
        }

        cache.insert(stone.clone(), s.len());
        break;
    }

    input.len()
    */
}

pub fn blink(input: &String) -> Vec<String> {
    let mut stones = input
        .split_ascii_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    let mut result = Vec::new();

    for stone in stones.iter_mut() {
        if *stone == "0" {
            result.push(String::from("1"));
            continue;
        }

        if stone.len() % 2 == 0 {
            let right = stone.split_off(stone.len() / 2);
            let right = right.trim_start_matches("0");
            result.push(stone.clone());

            if right.is_empty() {
                result.push(String::from("0"));
            } else {
                result.push(String::from(right));
            }
            continue;
        }

        let num = stone.parse::<u128>().unwrap();
        let num = num * 2024;
        *stone = num.to_string();
        result.push(stone.clone());
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    #[test]
    fn test_blink() {
        let mut input = "0 1 10 99 999";
        assert_eq!(blink_many(&mut input.to_string(), 1), 7);
    }

    #[test]
    fn test_blink_2() {
        let mut input = "125 17";
        assert_eq!(blink_many(&mut input.to_string(), 6), 22);
    }

    const TEST_INPUT: &str = indoc! {"
        125 17
    "};

    #[test]
    fn test_part1_example1() {
        let input = crate::parser::part1(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO");
        assert_eq!(part1(&input), "55312");
    }
}
