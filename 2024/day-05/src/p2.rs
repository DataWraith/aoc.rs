use crate::structs::*;

use utility_belt::prelude::*;

pub fn part2(input: &PuzzleInput) -> String {
    let mut dependencies = HashMap::new();
    let mut reverse_dependencies = HashMap::new();
    let mut sum = 0;

    for (p1, p2) in input.rules.iter() {
        dependencies.entry(*p2).or_insert_with(Vec::new).push(*p1);
        reverse_dependencies
            .entry(*p1)
            .or_insert_with(Vec::new)
            .push(*p2);
    }

    'outer: for pages in input.pages.iter() {
        let mut result = Vec::new();
        let mut fail = false;

        for page in pages.iter() {
            for previous in result.iter() {
                if dependencies
                    .get(previous)
                    .unwrap_or(&vec![])
                    .contains(&page)
                {
                    fail = true;
                }
            }

            result.push(*page);
        }

        if !fail && result.len() == pages.len() {
            continue 'outer;
        }

        let start = &pages[0];
        for start in pages.iter() {
            let mut seen = vec![];
            seen.push(*start);

            let mut successors = |n: &u32| {
                if seen.len() == pages.len() {
                    return vec![];
                }

                let mut s = vec![];

                'outer: for page in pages.iter() {
                    if seen.contains(page) {
                        continue;
                    }

                    for previous in seen.iter() {
                        if dependencies
                            .get(previous)
                            .unwrap_or(&vec![])
                            .contains(&page)
                        {
                            continue 'outer;
                        }
                    }

                    s.push(*page);
                }

                seen.extend(s.iter());

                //dbg!(&seen);

                //if let Some(deps) = dependencies.get(&n) {
                //for dep in deps.iter() {
                //if !seen.contains(dep) && pages.contains(dep) {
                //s.push(*dep);
                //}
                //}
                //}

                //for previous in seen.iter() {
                //if !dependencies.get(previous).unwrap_or(&vec![]).contains(&n) {
                //s = s.into_iter().filter(|p| *p != *n).collect_vec();
                //}
                //}

                //for successor in s.iter() {
                //seen.insert(*successor);
                //}

                s
            };

            let mut dfs = BrFS::new(vec![*start]);
            let mut visited = Vec::new();

            while let Some(n) = dfs.next(&mut successors) {
                if visited.len() == pages.len() {
                    break;
                }

                visited.push(n);
            }

            if visited.len() == pages.len() {
                sum += visited[visited.len() / 2];
                break;
            }
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
