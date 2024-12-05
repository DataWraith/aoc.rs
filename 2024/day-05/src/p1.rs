use crate::structs::*;

use utility_belt::prelude::*;

/*
pub fn part1(input: &PuzzleInput) -> String {
    let mut dependencies = HashMap::new();

    for (p1, p2) in input.rules.iter() {
        dependencies.entry(*p1).or_insert_with(Vec::new).push(*p2);
    }

    'outer: for pages in input.pages.iter().cloned() {
        let mut seen: HashSet<u32> = HashSet::new();

        for start in pages.iter() {
            seen.insert(*start);

            let mut successors = |n: &u32| {
                let mut s = vec![];

                if let Some(deps) = dependencies.get(&n) {
                    s.extend(deps.iter().copied());
                }

                if s.is_empty() {
                    s.extend(pages.iter().filter(|p| !seen.contains(p)));
                }

                let succ = s
                    .iter()
                    .filter(|p| !seen.contains(p) && pages.contains(p))
                    .copied()
                    .collect_vec();

                for successor in succ.iter() {
                    seen.insert(*successor);
                }

                succ
            };

            let mut dfs = BrFS::new(vec![*start]);
            let mut visited = Vec::new();

            while let Some(n) = dfs.next(&mut successors) {
                visited.push(n);
            }

            if visited.len() != pages.len() {
                continue 'outer;
            }

            dbg!(&pages);
        }
    }

    "".to_string()
}
    */

/*
#[tracing::instrument(skip(input))]
pub fn part1_a(input: &PuzzleInput) -> String {
    let edges: Vec<_> = input
        .rules
        .iter()
        .copied()
        .map(|(p1, p2)| (p1, p2, p1))
        .collect();

    dbg!(&edges);

    let g = DiGraph::<u32, u32>::from_edges(edges);
    let topo = petgraph::algo::toposort(&g, None).unwrap();
    let indices = topo.iter().map(|v| g[*v]).collect::<Vec<_>>();

    println!("{:?}", Dot::new(&g));

    for pages in input.pages.iter() {
        for page in pages.iter() {
            //dbg!(&indices.get(page as usize));
        }
    }

    "".to_string()
}
    */

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
