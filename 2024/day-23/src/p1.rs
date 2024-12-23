use utility_belt::prelude::*;

use crate::parser::*;

pub fn part1(input: &PuzzleInput) -> String {
    // This is a bit of a mouthful, but petgraph makes this surprisingly easy.
    //
    // 1. Iterate over all nodes that start with "t"
    // 2. Get all neighbors of the node
    // 3. Check all combinations of two neighbors if they are connected -- if so, we have a three-connected set
    // 4. Sort the three-connected set and return it as a vector
    // 5. At this point, we have a vector of vectors for each node that starts with "t",
    //    so we need to flatten it to remove one level of nesting
    // 6. Then we remove duplicates using .unique() and return the length of the result
    input
        .graph
        .nodes()
        .filter(|n| n.starts_with("t"))
        .map(|n| {
            input
                .graph
                .neighbors(n)
                .tuple_combinations()
                .filter(|(a, b)| input.graph.contains_edge(a, b))
                .map(|(a, b)| {
                    let mut triplet = vec![n, a, b];
                    triplet.sort();
                    triplet
                })
                .collect_vec()
        })
        .flatten()
        .unique()
        .count()
        .to_string()
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
    fn test_part1_example() {
        let input = crate::parser::part1(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(part1(&input), "7");
    }
}
