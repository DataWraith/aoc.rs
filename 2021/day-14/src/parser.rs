use utility_belt::prelude::*;

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub polymer: String,
    pub rules: HashMap<String, Vec<String>>,
}

pub fn part1(input: &'static str) -> PuzzleInput {
    let (polymer, rules) = input.split_once("\n\n").unwrap();
    let polymer = format!(" {} ", polymer);

    let rules = rules
        .lines()
        .map(|line| {
            let (pattern, result) = line.split_once(" -> ").unwrap();
            let (a, b) = (
                pattern.chars().nth(0).unwrap(),
                pattern.chars().nth(1).unwrap(),
            );

            (
                pattern.to_string(),
                vec![format!("{}{}", a, result), format!("{}{}", result, b)],
            )
        })
        .collect();

    PuzzleInput { polymer, rules }
}

pub fn part2(input: &'static str) -> PuzzleInput {
    part1(input)
}

pub const TEST_INPUT: &str = indoc! {"
    NNCB

    CH -> B
    HH -> N
    CB -> H
    NH -> C
    HB -> C
    HC -> B
    HN -> C
    NN -> C
    BH -> H
    NC -> B
    NB -> B
    BN -> B
    BB -> N
    BC -> B
    CC -> N
    CN -> C
"};
