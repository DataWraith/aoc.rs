use utility_belt::prelude::*;

use crate::parser::*;

pub fn part2(input: &PuzzleInput) -> String {
    let mut visited = HashSet::new();
    let mut best_clique = Vec::new();

    // This is pretty much a brute-force approach that follows directly from the
    // definition of a clique.
    //
    // We start with a single-node clique and try to grow it. To do that, we
    // iterate over all nodes that have not been processed yet and check if they
    // are connected to all nodes in the current clique.
    //
    // If they are connected to everyone in the clique, they are part of the
    // clique by definition, so we add them to the set and continue.
    //
    // Once a node is part of a clique, we add it to the visited set to avoid
    // processing it again -- Once a node is part of a clique, it can't be part
    // of another, larger clique: either the current clique grows to be the
    // largest possible, or another, larger clique is found which will not
    // contain the node. Note that this is not true in general, but it works
    // for this problem input.
    //
    // Not sure if this is just fast because the input is benign, or if it's
    // actually a sound algorithm...
    for start in input.graph.nodes() {
        if visited.contains(start) {
            continue;
        }

        let mut clique = Vec::from([start]);

        'outer: for node in input.graph.neighbors(start) {
            for c in clique.iter() {
                if !input.graph.contains_edge(c, node) {
                    continue 'outer;
                }
            }

            clique.push(node);
            visited.insert(node);
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
