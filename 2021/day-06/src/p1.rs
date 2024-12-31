use utility_belt::prelude::*;

use crate::parser::*;

pub fn part1(input: &PuzzleInput) -> String {
    let mut counter = Counter::from_iter(input.lanternfish.iter().cloned());

    for _ in 0..80 {
        counter = state_iteration(
            &counter,
            |fish, _| {
                if *fish == 0 {
                    vec![6, 8]
                } else {
                    vec![fish - 1]
                }
            },
            (),
        );
    }

    counter.values().sum::<usize>().to_string()
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
