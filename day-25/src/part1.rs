use crate::structs::*;

use rustworkx_core::connectivity::stoer_wagner_min_cut;

pub fn part1(input: &PuzzleInput) -> String {
    let num_nodes = input.graph.node_count();

    if let Ok(Some((_, v))) = stoer_wagner_min_cut(&input.graph, |_| Ok::<i32, ()>(1)) {
        return (v.len() * (num_nodes - v.len())).to_string();
    }

    unreachable!()
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
