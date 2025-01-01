use utility_belt::prelude::*;

use crate::parser::*;

pub fn part1(input: &PuzzleInput) -> String {
    let min_x = *input.positions.iter().min().unwrap();
    let max_x = *input.positions.iter().max().unwrap();
    let ideal_x = find_optimum(input, min_x, max_x, fuel_cost);

    fuel_cost(input, ideal_x).to_string()
}

pub fn find_optimum(
    input: &PuzzleInput,
    min_x: i64,
    max_x: i64,
    fuel_cost: fn(input: &PuzzleInput, x: usize) -> usize,
) -> usize {
    let prev_x = bisect(min_x as usize, max_x as usize + 1, |x| {
        // The predicate must flip from false to true at the ideal_x.
        //
        // This means that we can just check if the cost is increasing,
        // because that will be false before the ideal_x and true after.

        let x_cost = fuel_cost(input, *x);
        let next_cost = fuel_cost(input, x + 1);

        next_cost > x_cost
    })
    .unwrap();

    // We find the first position where the cost is decreasing, so prev_x is not
    // actually ideal -- we need to move one more to the right to get to the
    // minimum.
    prev_x + 1
}

fn fuel_cost(input: &PuzzleInput, x: usize) -> usize {
    input
        .positions
        .iter()
        .map(|y| x.abs_diff(*y as usize))
        .sum::<usize>()
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
        assert_eq!(part1(&input), "37");
    }
}
