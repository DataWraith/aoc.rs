use crate::{part1::press_button, structs::*};

use utility_belt::prelude::*;

pub fn part2(input: &PuzzleInput) -> String {
    let mut state = State::new(input);
    let necessary_conjunctions = ["tx", "dd", "nz", "ph"];

    loop {
        let (s, _low, _high) = press_button(input, state);
        state = s;

        if necessary_conjunctions
            .iter()
            .all(|&c| state.conjunction_cycles.contains_key(c))
        {
            return necessary_conjunctions
                .iter()
                .map(|&c| state.conjunction_cycles[c])
                .fold(1, lcm)
                .to_string();
        }
    }
}
