use utility_belt::prelude::*;

use crate::parser::*;

pub fn part1(input: &PuzzleInput) -> String {
    lanternfish_after_days(&input.lanternfish, 80).to_string()
}

pub fn lanternfish_after_days(fish: &[u64], days: usize) -> usize {
    let mut counter = Counter::from_iter(fish.iter().cloned());

    for _ in 0..days {
        counter = state_iteration(&counter, |fish, _| lanternfish_spawning(fish), ());
    }

    counter.values().sum::<usize>()
}

pub fn lanternfish_spawning(fish: &u64) -> Vec<u64> {
    if *fish == 0 {
        vec![6, 8]
    } else {
        vec![fish - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = indoc! {"
        3,4,3,1,2
    "};

    #[test]
    fn test_part1_example() {
        let input = crate::parser::part1(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(part1(&input), "5934");
    }
}
