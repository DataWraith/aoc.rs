use crate::structs::*;

use petgraph::{
    algo::all_simple_paths,
    graph::NodeIndex,
    graph::UnGraph,
    visit::{EdgeRef, IntoEdges},
};
use utility_belt::prelude::*;

pub fn part2(input: &PuzzleInput) -> String {
    let (graph, start, goal) = contract_graph(&input.grid);
    solve(graph, start, goal)
}

pub fn solve(graph: UnGraph<Coordinate, usize>, start: NodeIndex, goal: NodeIndex) -> String {
    let max_len = all_simple_paths(&graph, start, goal, 0, None)
        .map(|p: Vec<_>| {
            let mut sum = 0;

            for (n1, n2) in p.iter().tuple_windows() {
                let edge = graph.find_edge(*n1, *n2).unwrap();
                let weight = graph[edge];

                sum += weight;
            }

            sum
        })
        .max()
        .unwrap();

    (max_len - 1).to_string()
}

pub fn contract_graph(grid: &Grid2D<char>) -> (UnGraph<Coordinate, usize>, NodeIndex, NodeIndex) {
    fn follow_path(
        cur: Coordinate,
        prev: Coordinate,
        grid: &Grid2D<char>,
    ) -> (Coordinate, Coordinate, usize) {
        let mut cur = cur;
        let mut prev = prev;
        let mut len = 1;

        loop {
            let directions = valid_directions(cur, prev, grid);

            if directions.len() == 1 {
                prev = cur;
                cur += directions[0].into();
                len += 1;
            } else {
                break (cur, prev, len);
            }
        }
    }

    let mut adjacencies = HashMap::new();

    let mut queue = VecDeque::new();
    queue.push_back((Coordinate::new(1, 0), Coordinate::new(1, 0)));

    while let Some((cur, prev)) = queue.pop_front() {
        let (next, next_pred, len) = follow_path(cur, prev, grid);

        let adj = adjacencies.entry(prev).or_insert_with(Vec::new);

        if adj.iter().any(|(c, _)| *c == next) {
            continue;
        }

        adj.push((next, len));

        let directions = valid_directions(next, next_pred, grid);

        for dir in directions {
            queue.push_back((next + dir.into(), next));
        }
    }

    let mut graph = UnGraph::<Coordinate, usize>::new_undirected();
    let mut ids = HashMap::default();

    for (cur, edges) in adjacencies.iter().sorted_by_key(|(k, v)| (k.y(), k.x())) {
        let cur = *ids.entry(cur).or_insert_with(|| graph.add_node(*cur));

        for (succ, length) in edges {
            let next = *ids.entry(succ).or_insert_with(|| graph.add_node(*succ));

            if !graph.contains_edge(cur, next) {
                graph.add_edge(cur, next, *length);
            }
        }
    }

    let start = graph.node_indices().find(|&n| graph[n].y() == 0).unwrap();
    let goal = graph
        .node_indices()
        .find(|&n| graph[n].y() == grid.height() as i32 - 1)
        .unwrap();

    (graph, start, goal)
}

fn valid_directions(cur: Coordinate, prev: Coordinate, grid: &Grid2D<char>) -> Vec<Direction> {
    Direction::all()
        .filter(move |&dir| {
            let neighbor = cur + dir.into();

            if neighbor == prev {
                return false;
            }

            grid.get(neighbor).is_some() && grid[neighbor] != '#'
        })
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::indoc;

    const TEST_INPUT: &str = indoc! {"
        #.#####################
        #.......#########...###
        #######.#########.#.###
        ###.....#.>.>.###.#.###
        ###v#####.#v#.###.#.###
        ###.>...#.#.#.....#...#
        ###v###.#.#.#########.#
        ###...#.#.#.......#...#
        #####.#.#.#######.#.###
        #.....#.#.#.......#...#
        #.#####.#.#.#########v#
        #.#...#...#...###...>.#
        #.#.#v#######v###.###v#
        #...#.>.#...>.>.#.###.#
        #####v#.#.###v#.#.###.#
        #.....#...#...#.#.#...#
        #.#########.###.#.#.###
        #...###...#...#...#.###
        ###.###.#.###v#####v###
        #...#...#.#.>.>.#.>.###
        #.###.###.#.###.#.#v###
        #.....###...###...#...#
        #####################.#
    "};

    #[test]
    fn test_part2() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part2(&input), "154");
    }
}
