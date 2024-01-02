use crate::structs::*;

use utility_belt::prelude::*;

pub fn part1(input: &PuzzleInput) -> String {
    let xs = input
        .instructions
        .iter()
        .scan((0, 1), |mut acc, (cycles, delta)| {
            acc.0 += cycles;
            acc.1 += delta;
            Some(acc.clone())
        })
        .collect::<Vec<_>>();

    let mut signal_strength = 0;

    for check_cycle in (20..).step_by(40) {
        let idx = bisect(0, xs.len(), |i| xs[i].0 >= check_cycle);

        if idx == xs.len() {
            break;
        }

        signal_strength += check_cycle as isize * xs[idx - 1].1;
    }

    signal_strength.to_string()
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
