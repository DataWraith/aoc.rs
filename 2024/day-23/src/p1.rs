use utility_belt::prelude::*;

use crate::parser::*;

pub fn part1(input: &PuzzleInput) -> String {
    let three_connected = three_connected(&input.connections);

    let mut count = 0;

    for nodes in three_connected.into_iter() {
        let node1 = &nodes[0];
        let node2 = &nodes[1];
        let node3 = &nodes[2];

        if !node1.starts_with("t") && !node2.starts_with("t") && !node3.starts_with("t") {
            continue;
        }

        count += 1;
    }

    count.to_string()
}

pub fn three_connected(conns: &HashMap<String, HashSet<String>>) -> HashSet<Vec<String>> {
    let mut three_connected = HashSet::new();

    for (node1, connections) in conns.iter() {
        for (node2, connections2) in conns.iter() {
            if node2 == node1 {
                continue;
            }

            if !connections.contains(node2) {
                continue;
            }

            if !connections2.contains(node1) {
                continue;
            }

            let intersection = connections
                .intersection(connections2)
                .collect::<HashSet<_>>();

            for node3 in intersection {
                let mut vec = vec![node1.clone(), node2.clone(), node3.clone()];
                vec.sort();
                three_connected.insert(vec);
            }
        }
    }

    three_connected
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
