use crate::structs::*;

use utility_belt::prelude::*;

pub fn part1(input: &PuzzleInput) -> String {
    let mut sum = 0;

    let can_precede = |a: &u64, b: &u64| {
        !input
            .dependencies
            .get(a)
            .map(|deps| deps.contains(b))
            .unwrap_or(false)
    };

    for pages in input.pages.iter() {
        if pages.is_sorted_by(can_precede) {
            sum += pages[pages.len() / 2];
        }
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = utility_belt::prelude::indoc! {"
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
