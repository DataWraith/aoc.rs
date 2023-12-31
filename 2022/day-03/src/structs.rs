use utility_belt::prelude::*;

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub rucksacks: Vec<Rucksack>,
}

#[derive(Clone, Debug)]
pub struct Rucksack {
    pub left_compartment: Vec<char>,
    pub right_compartment: Vec<char>,
}
