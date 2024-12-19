use ndarray::Array1;
use num::{Rational64, Zero};
use utility_belt::prelude::*;

use crate::parser::*;

pub fn part1(input: &PuzzleInput) -> String {
    input
        .games
        .iter()
        .filter_map(|g| calculate_num_button_presses(g.clone(), input.offset))
        .map(|(a, b)| 3 * a + b)
        .sum::<i64>()
        .to_string()
}

fn calculate_num_button_presses(claw_game: ClawGame, offset: i64) -> Option<(i64, i64)> {
    let mut matrix = Array2::zeros((2, 3));
    let mut answer: Array1<Rational64> = Array1::zeros(2).view().to_owned();

    // f64 is not enough precision for this problem apparently, so we need to
    // use BigRational instead. That's fairly slow, unfortunately.
    matrix[[0, 0]] = Rational64::from_integer(claw_game.offset_a.x);
    matrix[[0, 1]] = Rational64::from_integer(claw_game.offset_b.x);
    matrix[[0, 2]] = Rational64::from_integer(claw_game.prize.x + offset);
    matrix[[1, 0]] = Rational64::from_integer(claw_game.offset_a.y);
    matrix[[1, 1]] = Rational64::from_integer(claw_game.offset_b.y);
    matrix[[1, 2]] = Rational64::from_integer(claw_game.prize.y + offset);

    let soln = gauss_jordan(matrix, &mut answer, Rational64::zero());

    if soln != Solution::Unique {
        return None;
    }

    // Make sure that we get integer coordinates -- fractional coordinates don't count as hit.
    if !answer[0].is_integer() || !answer[1].is_integer() {
        return None;
    }

    Some((*answer[0].numer(), *answer[1].numer()))
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
