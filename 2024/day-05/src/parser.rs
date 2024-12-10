use utility_belt::prelude::*;

use crate::structs::*;

pub fn part1(input: &str) -> PuzzleInput {
    let mut ruleset = HashSet::new();
    let mut page_sets = Vec::new();
    let (rules, pages) = input.split_once("\n\n").unwrap();

    for line in rules.lines() {
        let (p1, p2) = line.split_once("|").unwrap();
        let p1: u32 = p1.parse().unwrap();
        let p2: u32 = p2.parse().unwrap();
        ruleset.insert((p1, p2));
    }

    for line in pages.lines() {
        let pages: Vec<u32> = line.split(',').map(|s| s.parse().unwrap()).collect();
        page_sets.push(pages);
    }

    PuzzleInput {
        rules: ruleset,
        pages: page_sets,
    }
}

pub fn part2(input: &str) -> PuzzleInput {
    part1(input)
}
