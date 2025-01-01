use crate::{p1::find_optimum, parser::*};

pub fn part2(input: &PuzzleInput) -> String {
    let min_x = *input.positions.iter().min().unwrap();
    let max_x = *input.positions.iter().max().unwrap();
    let ideal_x = find_optimum(input, min_x, max_x, fuel_cost);

    fuel_cost(input, ideal_x).to_string()
}

fn fuel_cost(input: &PuzzleInput, x: i64) -> u64 {
    input
        .positions
        .iter()
        .map(|y| x.abs_diff(*y))
        .map(|dist| dist * (dist + 1) / 2)
        .sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = indoc! {"
        16,1,2,0,4,2,7,1,2,14
    "};

    #[test]
    fn test_part2_example() {
        let input = crate::parser::part2(TEST_INPUT);
        assert_ne!(TEST_INPUT.trim(), "TODO");
        assert_eq!(part2(&input), "168");
    }
}
