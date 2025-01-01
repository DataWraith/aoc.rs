use utility_belt::prelude::*;

use crate::parser::*;

pub fn part1(input: &PuzzleInput) -> String {
    let min_x = *input.positions.iter().min().unwrap();
    let max_x = *input.positions.iter().max().unwrap();

    let fuel_cost = (min_x..=max_x)
        .map(|x| input.positions.iter().map(|y| (x - y).abs()).sum::<i64>())
        .min()
        .unwrap();

    fuel_cost.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = indoc! {"
        16,1,2,0,4,2,7,1,2,14
    "};

    #[test]
    fn test_part1_example() {
        let input = crate::parser::part1(TEST_INPUT);
        assert_ne!(TEST_INPUT.trim(), "TODO");
        assert_eq!(part1(&input), "TODO");
    }
}
