use crate::structs::*;

pub fn parse(input: &str) -> PuzzleInput {
    let mut calories: Vec<Vec<usize>> = Vec::new();
    let mut cur = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            calories.push(cur);
            cur = Vec::new();
            continue;
        }

        let calories = line.parse::<usize>().unwrap();
        cur.push(calories);
    }

    if !cur.is_empty() {
        calories.push(cur);
    }

    PuzzleInput { calories }
}
