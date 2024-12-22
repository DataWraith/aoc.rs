use utility_belt::prelude::*;

use crate::parser::*;

pub fn part1(input: &PuzzleInput) -> String {
    todo!("day_22::p1::part1");
}

pub fn rng(state: u64) -> u64 {
    ((state * 64) >> 5)
}

pub fn mix

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = indoc! {"
15887950
16495136
527345
704524
1553684
12683156
11100544
12249484
7753432
5908254
"};

    #[test]
    fn test_part1_example() {
        let input = crate::parser::part1(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(part1(&input), "TODO");
    }
}
