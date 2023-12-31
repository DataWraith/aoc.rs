use utility_belt::prelude::*;

use crate::structs::*;

pub fn parse(input: &str) -> PuzzleInput {
    let mut rucksacks = Vec::new();

    for line in input.lines() {
        let chars = line.chars().collect::<Vec<char>>();
        assert!(chars.len() % 2 == 0);

        let left_compartment = 0..(chars.len() / 2);
        let right_compartment = (chars.len() / 2)..chars.len();

        rucksacks.push(Rucksack {
            left_compartment: HashSet::from_iter(chars[left_compartment].iter().cloned()),
            right_compartment: HashSet::from_iter(chars[right_compartment].iter().cloned()),
        });
    }

    PuzzleInput { rucksacks }
}
