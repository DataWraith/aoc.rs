use utility_belt::prelude::*;

use crate::structs::*;

pub fn parse(input: &str) -> PuzzleInput {
    let mut rucksacks = Vec::new();

    for line in input.lines() {
        let chars = line.chars().collect::<Vec<char>>();
        assert!(chars.len() % 2 == 0);

        rucksacks.push(Rucksack {
            left_compartment: chars[0..(chars.len() / 2)].to_vec(),
            right_compartment: chars[(chars.len() / 2)..chars.len()].to_vec(),
        });
    }

    PuzzleInput { rucksacks }
}
