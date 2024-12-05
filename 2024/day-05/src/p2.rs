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

        let search = SearchState {
            chosen: vec![],
            remaining: pages.clone(),
        };

        let mut successors = |n: &SearchState| {
            let mut succ = vec![];

            'outer: for page in n.remaining.iter() {
                for previous in n.chosen.iter() {
                    if dependencies
                        .get(previous)
                        .unwrap_or(&vec![])
                        .contains(&page)
                    {
                        continue 'outer;
                    }
                }

                let mut cloned = n.chosen.clone();
                cloned.push(*page);

                succ.push(SearchState {
                    chosen: cloned,
                    remaining: n
                        .remaining
                        .iter()
                        .cloned()
                        .filter(|p| p != page)
                        .collect_vec(),
                });
            }

            succ
        };

        let mut dfs = BrFS::new(vec![search]);
        let mut visited = Vec::new();

        while let Some(n) = dfs.next(&mut successors) {
            visited.push(n);
        }

        let chosen = visited.last().unwrap().chosen.clone();

        if chosen.len() == pages.len() {
            dbg!(i, input.pages.len(), &chosen);
            sum += chosen[chosen.len() / 2];
        }
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
