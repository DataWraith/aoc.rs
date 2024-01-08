use std::ops::Range;

use crate::structs::*;

use utility_belt::prelude::*;

pub fn part1(input: &PuzzleInput) -> String {
    input
        .races
        .iter()
        .map(winning_race_strategies)
        .map(|rng| (rng.end + 1) - rng.start)
        .product::<usize>()
        .to_string()
}

pub fn winning_race_strategies(race: &Race) -> Range<usize> {
    let min_hold = bisect(0, race.time / 2, |t| {
        boat_distance(race, *t) > race.distance
    })
    .unwrap();

    let max_hold = bisect(min_hold + 1, race.time, |t| {
        boat_distance(race, *t) <= race.distance
    })
    .unwrap();

    min_hold..(max_hold - 1)
}

pub fn boat_distance(race: &Race, button_held_for: usize) -> usize {
    let speed = button_held_for;
    let remaining_time = race.time - button_held_for;

    speed * remaining_time
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::indoc;

    const TEST_INPUT: &str = indoc! {"
        Time:      7  15   30
        Distance:  9  40  200
    "};

    #[test]
    fn test_part1() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part1(&input), "288");
    }
}
