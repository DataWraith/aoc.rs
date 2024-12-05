use crate::{p1::check, structs::*};

use utility_belt::prelude::*;

#[derive(Debug, Clone)]
struct SearchState {
    chosen: Vec<u32>,
    remaining: Vec<u32>,
}

pub fn part2(input: &PuzzleInput) -> String {
    let mut dependencies = HashMap::new();
    let mut sum = 0;

    for (p1, p2) in input.rules.iter() {
        dependencies.entry(*p2).or_insert_with(Vec::new).push(*p1);
    }

    for (i, pages) in input.pages.iter().enumerate() {
        if check(pages, &dependencies) {
            continue;
        };

        let mut chosen = pages.clone();
        chosen.sort_by(|a, b| {
            if dependencies.get(b).unwrap_or(&vec![]).contains(&a) {
                return std::cmp::Ordering::Less;
            }

            std::cmp::Ordering::Greater
        });

        sum += chosen[chosen.len() / 2];
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = indoc! {"
        47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13

        75,47,61,53,29
        97,61,53,29,13
        75,29,13
        75,97,47,61,53
        61,13,29
        97,13,75,29,47
    "};

    #[test]
    fn test_part2_example() {
        let input = crate::parser::part2(TEST_INPUT);
        assert_eq!(part2(&input), "123");
    }
}
