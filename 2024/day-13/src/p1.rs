use cached::proc_macro::cached;
use glam::U64Vec2;
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
    if claw_game.prize.x == 0 && claw_game.prize.y == 0 {
        return Some((0, 0));
    }

    if claw_game.prize.x == claw_game.offset_a.x && claw_game.prize.y == claw_game.offset_a.y {
        return Some((1, 0));
    }

    if claw_game.prize.x == claw_game.offset_b.x && claw_game.prize.y == claw_game.offset_b.y {
        return Some((0, 1));
    }

    let mut result: Option<(usize, usize)> = None;
    let mut result2: Option<(usize, usize)> = None;

    let a_presses_x = gcd(claw_game.prize.x, claw_game.offset_a.x);
    let a_presses_y = gcd(claw_game.prize.y, claw_game.offset_a.y);
    let a_presses = if a_presses_x == 1 {
        a_presses_y
    } else {
        a_presses_x
    };
    let a_presses = a_presses.max(1);

    for ap in (1..=a_presses).rev() {
        let progress = U64Vec2::new(ap * claw_game.offset_a.x, ap * claw_game.offset_a.y);

        if claw_game.prize.x >= progress.x && claw_game.prize.y >= progress.y {
            let new_game = ClawGame {
                prize: claw_game.prize - progress,
                ..claw_game
            };

            if let Some((a, b)) = calculate_num_button_presses(new_game) {
                if result.is_none()
                    || result.unwrap().0 * 3 + result.unwrap().1
                        > (ap as usize + a) * 3 + b as usize
                {
                    result = Some((a + ap as usize, b));
                }
            }
        }
    }

    let b_presses_x = gcd(claw_game.prize.x, claw_game.offset_b.x);
    let b_presses_y = gcd(claw_game.prize.y, claw_game.offset_b.y);
    let b_presses = if b_presses_x == 1 {
        b_presses_y
    } else {
        b_presses_x
    };
    let b_presses = b_presses.max(1);

    for bp in (1..=b_presses).rev() {
        let progress = U64Vec2::new(bp * claw_game.offset_b.x, bp * claw_game.offset_b.y);

        if claw_game.prize.x >= progress.x && claw_game.prize.y >= progress.y {
            let new_game = ClawGame {
                prize: claw_game.prize - progress,
                ..claw_game
            };

            if let Some((a, b)) = calculate_num_button_presses(new_game) {
                if result2.is_none()
                    || result2.unwrap().0 * 3 + result2.unwrap().1 > a * 3 + b + bp as usize
                {
                    result2 = Some((a, b + bp as usize));
                }
            }
        }
    }

    if result.is_none() && result2.is_none() {
        return None;
    }

    if result.is_none() {
        return result2;
    }

    if result2.is_none() {
        return result;
    }

    if result.unwrap().0 * 3 + result.unwrap().1 > result2.unwrap().0 * 3 + result2.unwrap().1 {
        return result2;
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
