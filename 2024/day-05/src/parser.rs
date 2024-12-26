use utility_belt::prelude::*;

use crate::structs::*;

pub fn part1(input: &str) -> PuzzleInput {
    let (rules, pages) = input.split_once("\n\n").unwrap();

    let page_sets = pages.lines().map(|line| parse_uints(line)).collect();

    let page_dependencies = parse_uints(rules);
    let mut dependencies = HashMap::new();

    for w in page_dependencies.chunks(2) {
        let (p1, p2) = (w[0], w[1]);

        dependencies
            .entry(p2)
            .or_insert_with(HashSet::new)
            .insert(p1);
    }

    PuzzleInput {
        dependencies,
        pages: page_sets,
    }
}

pub fn part2(input: &str) -> PuzzleInput {
    part1(input)
}
