use glam::U64Vec2;
use utility_belt::prelude::*;

use crate::parser::*;

#[tracing::instrument(skip(input))]
pub fn part2(input: &PuzzleInput) -> String {
    let mut new_games = vec![];

    for game in input.games.iter() {
        new_games.push(ClawGame {
            prize: game.prize + U64Vec2::new(10000000000000,10000000000000),
            ..*game
        })
    }

    crate::p1::part1(&PuzzleInput { games: new_games })
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
    fn test_part2_example() {
        let input = crate::parser::part2(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO");
        assert_eq!(part2(&input), "TODO");
    }
}
