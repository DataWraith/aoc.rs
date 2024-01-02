use crate::structs::*;

use utility_belt::prelude::*;

pub fn part1(input: &PuzzleInput) -> String {
    let xs = register_sequence(input);

    let mut signal_strength = 0;

    for check_cycle in (20..xs.len()).step_by(40) {
        signal_strength += check_cycle as isize * xs[check_cycle - 1];
    }

    signal_strength.to_string()
}

pub fn register_sequence(input: &PuzzleInput) -> Vec<isize> {
    let mut prev_x = 1;

    let xs = std::iter::once(1)
        .chain(input.instructions.iter().flat_map(|(cycles, delta)| {
            let mut v = Vec::new();

            for _ in 1..*cycles {
                v.push(prev_x);
            }

            v.push(prev_x + delta);
            prev_x += delta;

            v
        }))
        .collect_vec();

    xs
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part1(&input), "13140");
    }
}
