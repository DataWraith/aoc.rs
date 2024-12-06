use crate::structs::*;

use utility_belt::prelude::*;

pub fn part1(input: &PuzzleInput) -> String {
    let mut dependencies = HashMap::new();
    let mut sum = 0;

    for (p1, p2) in input.rules.iter() {
        dependencies.entry(*p2).or_insert_with(Vec::new).push(*p1);
    }

    for pages in input.pages.iter() {
        if check(pages, &dependencies) {
            sum += pages[pages.len() / 2];
        };
    }

    sum.to_string()
}

pub fn check(pages: &Vec<u32>, dependencies: &HashMap<u32, Vec<u32>>) -> bool {
    let mut result = Vec::new();

    for page in pages.iter() {
        for previous in result.iter() {
            if dependencies
                .get(previous)
                .unwrap_or(&vec![])
                .contains(&page)
            {
                return false;
            }
        }

        result.push(*page);
    }

    if result.len() == pages.len() {
        return true;
    }

    false
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
    fn test_part1_example() {
        let input = crate::parser::part1(TEST_INPUT);
        assert_eq!(part1(&input), "143");
    }
}