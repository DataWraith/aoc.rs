use ndarray::Array1;
use num::{BigInt, BigRational, Zero};
use utility_belt::prelude::*;

use crate::parser::*;

#[tracing::instrument(skip(input))]
pub fn part1(input: &PuzzleInput) -> String {
    let result = input
        .games
        .iter()
        .filter_map(|g| calculate_num_button_presses(g.clone(), input.offset))
        .collect_vec();

    result
        .iter()
        .map(|(a, b)| 3 * a + b)
        .sum::<BigInt>()
        .to_string()
}

fn calculate_num_button_presses(claw_game: ClawGame, offset: u64) -> Option<(BigInt, BigInt)> {
    let mut matrix = Array2::zeros((2, 3));
    let mut answer: Array1<BigRational> = Array1::zeros(2).view().to_owned();

    // f64 is not enough precision for this problem apparently, so we need to
    // use BigRational instead. That's fairly slow, unfortunately.
    matrix[[0, 0]] = BigRational::new((claw_game.offset_a.x).into(), 1.into());
    matrix[[0, 1]] = BigRational::new((claw_game.offset_b.x).into(), 1.into());
    matrix[[0, 2]] = BigRational::new((claw_game.prize.x + offset).into(), 1.into());
    matrix[[1, 0]] = BigRational::new((claw_game.offset_a.y).into(), 1.into());
    matrix[[1, 1]] = BigRational::new((claw_game.offset_b.y).into(), 1.into());
    matrix[[1, 2]] = BigRational::new((claw_game.prize.y + offset).into(), 1.into());

    let soln = gauss_jordan(matrix, &mut answer, BigRational::zero());

    if soln != Solution::Unique {
        return None;
    }

    // Make sure that we get integer coordinates -- fractional coordinates don't count as hit.
    if *answer[0].denom() != BigInt::from(1) || *answer[1].denom() != BigInt::from(1) {
        return None;
    }

    Some((answer[0].to_integer(), answer[1].to_integer()))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = indoc! {"
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
"};

    #[test]
    fn test_part1_example() {
        let input = crate::parser::part1(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO");
        assert_eq!(part1(&input), "480");
    }
}
