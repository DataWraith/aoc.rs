use cached::proc_macro::cached;
use utility_belt::prelude::*;

use crate::parser::*;

#[tracing::instrument(skip(input))]
pub fn part1(input: &PuzzleInput) -> String {
    let result = input
        .games
        .iter()
        .filter_map(|g| calculate_num_button_presses(g.clone()))
        .collect_vec();
    result
        .iter()
        .map(|(a, b)| 3 * a + b)
        .sum::<usize>()
        .to_string()
}

#[cached]
fn calculate_num_button_presses(claw_game: ClawGame) -> Option<(usize, usize)> {
    if claw_game.prize == Coordinate::new(0, 0) {
        return Some((0, 0));
    }

    let mut min = usize::MAX;
    let mut result = None;

    if claw_game.prize >= claw_game.offset_a {
        let a_game = ClawGame {
            prize: claw_game.prize - claw_game.offset_a,
            ..claw_game
        };

        if let Some(m) = calculate_num_button_presses(a_game).map(|(a, b)| (a + 1, b)) {
            let c = m.0 * 3 + m.1;

            if c < min {
                min = c;
                result = Some(m);
            }
        }
    }

    if claw_game.prize >= claw_game.offset_b {
        let b_game = ClawGame {
            prize: claw_game.prize - claw_game.offset_b,
            ..claw_game
        };

        if let Some(m) = calculate_num_button_presses(b_game).map(|(a, b)| (a, b + 1)) {
            let c = m.0 * 3 + m.1;

            if c < min {
                min = c;
                result = Some(m);
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

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
