use crate::parser::*;

pub fn part1(input: &PuzzleInput) -> String {
    let mut sum = 0;

    for seed in input.numbers.iter() {
        sum += std::iter::successors(Some(*seed), |&seed| Some(hash(seed)))
            .nth(2000)
            .unwrap();
    }

    sum.to_string()
}

pub fn hash(x: u64) -> u64 {
    let mut x = x;

    x = (x ^ (x * 64)) % 16777216;
    x = (x ^ (x / 32)) % 16777216;
    x = (x ^ (x * 2048)) % 16777216;
    x
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = indoc! {"
1
10
100
2024
"};

    #[test]
    fn test_part1_example() {
        let input = crate::parser::part1(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(part1(&input), "37327623");
    }
}
