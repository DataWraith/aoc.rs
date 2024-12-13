use utility_belt::prelude::*;
use z3::{
    ast::{Ast, Int},
    SatResult, Solver,
};

use crate::parser::*;

#[tracing::instrument(skip(input))]
pub fn part1(input: &PuzzleInput) -> String {
    let result = input
        .games
        .iter()
        .filter_map(|g| calculate_num_button_presses(g.clone(), input.part2))
        .collect_vec();

    result
        .iter()
        .map(|(a, b)| 3 * a + b)
        .sum::<usize>()
        .to_string()
}

fn calculate_num_button_presses(claw_game: ClawGame, part2: bool) -> Option<(usize, usize)> {
    let mut threshold = u64::MAX;
    let mut previous = None;

    for i in 0.. {
        let cfg = z3::Config::new();
        let ctx = z3::Context::new(&cfg);

        let a = Int::new_const(&ctx, "a");
        let b = Int::new_const(&ctx, "b");

        let prize_x = Int::from_u64(&ctx, claw_game.prize.x as u64);
        let prize_y = Int::from_u64(&ctx, claw_game.prize.y as u64);
        let t = Int::from_u64(&ctx, threshold);
        let m = if part2 {
            Int::from_u64(&ctx, 10000000000000)
        } else {
            Int::from_u64(&ctx, 0)
        };

        let solver = Solver::new(&ctx);

        solver
            .assert(&(&a * claw_game.offset_a.x + &b * claw_game.offset_b.x)._eq(&(&m + &prize_x)));
        solver
            .assert(&(&a * claw_game.offset_a.y + &b * claw_game.offset_b.y)._eq(&(&m + &prize_y)));
        solver.assert(&(&a * 3u64 + &b).lt(&t));

        let result = solver.check();

        if result == SatResult::Unsat {
            return previous;
        }

        let a = solver
            .get_model()
            .unwrap()
            .eval(&a, true)
            .unwrap()
            .as_u64()
            .unwrap();

        let b = solver
            .get_model()
            .unwrap()
            .eval(&b, true)
            .unwrap()
            .as_u64()
            .unwrap();

        previous = Some((a as usize, b as usize));
        threshold = a * 3 + b;
    }

    previous
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
