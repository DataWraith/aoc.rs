use crate::structs::*;

use ::petgraph::{graph::UnGraph, visit::EdgeRef};
use utility_belt::prelude::*;

pub fn part1(input: &PuzzleInput) -> String {
    // Find the three edges with the highest traffic when doing a BrFS, and
    // remove them from the graph.
    let graph = cut_graph(input);

    // Now we need to find the size of one of the connected components, which is
    // simple to do using DFS.
    let component_size = find_component_size(graph);

    // Calculate the product of the size of the two connected components. Since
    // there are exactly two, we can infer the size of one from the other one.
    let num_nodes = input.graph.node_count();

    (component_size * (num_nodes - component_size)).to_string()
}

fn find_component_size(graph: UnGraph<String, ()>) -> usize {
    // Do a DFS to find the size of one of the connected components
    let mut seen = HashSet::default();

    let mut successors = |idx: &petgraph::NodeIndex| {
        let mut result = Vec::new();

        if seen.insert(*idx) {
            for n in graph.neighbors(*idx) {
                result.push(n);
            }
        }

        result
    };

    let mut dfs = DFS::new(graph.node_indices().next().unwrap());
    let mut component_size = 0;

    while dfs.next(&mut successors).is_some() {
        component_size += 1;
    }

    component_size
}

fn cut_graph(input: &PuzzleInput) -> UnGraph<String, ()> {
    let mut edge_count = HashMap::default();
    let mut seen = HashSet::default();

    // We do a BrFS from each node, counting the number of times each edge is
    // seen. Then we remove the top 3 edges, because the edges with the most
    // traffic are the ones that need to be cut to split the graph into two
    // components. The reason for that is that all paths between the two
    // components _must_ go through one of those edges, so they'll accumulate
    // more traffic than the other edges.
    let mut successors = |idx: &petgraph::NodeIndex| {
        let mut result = Vec::new();

        seen.insert(*idx);

        for edge in input.graph.edges(*idx) {
            if seen.contains(&edge.target()) {
                continue;
            }

            edge_count
                .entry(edge.id())
                .and_modify(|c| *c += 1)
                .or_insert(1);

            result.push(edge.target());
        }

        // This is a bit of a hack because we can't clear the seen set from
        // outside between searches.
        if result.is_empty() {
            seen.clear();
        }

        result
    };

    // Actually run the searches
    for node in input.graph.node_indices() {
        let mut brfs = BrFS::new(vec![node]);

        while brfs.next(&mut successors).is_some() {}
    }

    // Find the edges to remove...
    let cut_edges = edge_count
        .iter()
        .sorted_by_key(|(_, v)| std::cmp::Reverse(**v))
        .take(3);

    // ... and remove them form a copy of the graph
    let mut graph = input.graph.clone();

    for cut_edge in cut_edges.into_iter() {
        graph.remove_edge(*cut_edge.0);
    }

    graph
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::indoc;

    const TEST_INPUT: &str = indoc! {"
        jqt: rhn xhk nvd
        rsh: frs pzl lsr
        xhk: hfx
        cmg: qnr nvd lhk bvb
        rhn: xhk bvb hfx
        bvb: xhk hfx
        pzl: lsr hfx nvd
        qnr: nvd
        ntq: jqt hfx bvb xhk
        nvd: lhk
        lsr: lhk
        rzs: qnr cmg lsr rsh
        frs: qnr lhk lsr
    "};

    #[test]
    fn test_part1() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part1(&input), "54");
    }
}
