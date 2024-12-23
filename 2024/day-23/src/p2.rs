use utility_belt::prelude::*;

use crate::parser::*;

pub fn part2(input: &PuzzleInput) -> String {
    let mut best_clique = Vec::new();

    let mut visited = HashSet::new();

    for start in input.connections.keys() {
        if visited.contains(start) {
            continue;
        }

        let mut clique = Vec::from([start.clone()]);

        'outer: for (node, connections) in input.connections.iter() {
            for c in clique.iter() {
                if !connections.contains(c) {
                    continue 'outer;
                }
            }

            clique.push(node.clone());
            visited.insert(node.clone());
        }

        if clique.len() > best_clique.len() {
            best_clique = clique;
        }
    }

    best_clique.iter().sorted().join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = utility_belt::prelude::indoc! {"
        kh-tc
        qp-kh
        de-cg
        ka-co
        yn-aq
        qp-ub
        cg-tb
        vc-aq
        tb-ka
        wh-tc
        yn-cg
        kh-ub
        ta-co
        de-co
        tc-td
        tb-wq
        wh-td
        ta-ka
        td-qp
        aq-cg
        wq-ub
        ub-vc
        de-ta
        wq-aq
        wq-vc
        wh-yn
        ka-de
        kh-ta
        co-tc
        wh-qp
        tb-vc
        td-yn
    "};

    #[test]
    fn test_part2_example() {
        let input = crate::parser::part2(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(part2(&input), "co,de,ka,ta");
    }
}
