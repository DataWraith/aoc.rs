use utility_belt::prelude::*;

use crate::parser::*;

pub fn part1(input: &PuzzleInput) -> String {
    let mut counter = Counter::new();

    for (a, b) in input.polymer.chars().tuple_windows() {
        let pattern = format!("{}{}", a, b);
        counter.add(pattern);
    }

    for _ in 0..10 {
        counter = state_iteration(
            &counter,
            |pattern, _| {
                if let Some(results) = input.rules.get(pattern) {
                    results.iter().map(|result| result.clone()).collect()
                } else {
                    vec![pattern.clone()]
                }
            },
            (),
        );
    }

    dbg!(&counter);

    let mut singles = Counter::new();

    for (pattern, count) in counter.iter() {
        let a = pattern.chars().nth(0).unwrap();
        let b = pattern.chars().nth(1).unwrap();

        if a != ' ' {
            singles.add_many(a, *count);
        }

        if b != ' ' {
            singles.add_many(b, *count);
        }
    }

    dbg!(&singles);

    let least_common = singles.values().min().unwrap() / 2;
    let most_common = singles.values().max().unwrap() / 2;

    (most_common - least_common).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;
    use utility_belt::prelude::*;

    #[test]
    fn test_part1_example() {
        let input = parser::part1(parser::TEST_INPUT);
        assert_ne!(parser::TEST_INPUT.trim(), "TODO");
        assert_eq!(part1(&input), "TODO");
    }
}
