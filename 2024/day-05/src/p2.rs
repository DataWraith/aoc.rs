use crate::structs::*;

use utility_belt::prelude::*;

pub fn part2(input: &PuzzleInput) -> String {
    let mut sum = 0;

    let mut dependencies = HashMap::new();

    for (p1, p2) in input.rules.iter() {
        dependencies.entry(*p2).or_insert_with(Vec::new).push(*p1);
    }

    let can_precede = |a: &u32, b: &u32| !dependencies.get(a).unwrap_or(&vec![]).contains(b);

    for mut pages in input.pages.iter().cloned() {
        if pages.is_sorted_by(can_precede) {
            continue;
        };

        pages.sort_by(|a, b| {
            if can_precede(a, b) {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        });

        sum += pages[pages.len() / 2];
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
    fn test_part2_example() {
        let input = crate::parser::part2(TEST_INPUT);
        assert_eq!(part2(&input), "123");
    }
}
