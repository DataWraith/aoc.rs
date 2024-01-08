use crate::{part1::follow_path, structs::*};

use ::petgraph::{algo::all_simple_paths, graph::NodeIndex, graph::UnGraph, visit::EdgeRef};
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
    let mut graph = UnGraph::<Coordinate, usize>::new_undirected();
    let mut node_ids = HashMap::default();

    let mut queue = VecDeque::new();
    queue.push_back((Coordinate::new(1, 0), Coordinate::new(1, 0)));

    'bfs: while let Some((cur, prev)) = queue.pop_front() {
        let (next, next_pred, len) = follow_path(cur, prev, grid, valid_directions);

        let prev_node = *node_ids.entry(prev).or_insert_with(|| graph.add_node(prev));
        let next_node = *node_ids.entry(next).or_insert_with(|| graph.add_node(next));

        for edge in graph.edges(prev_node) {
            if edge.target() == next_node {
                continue 'bfs;
            }
        }

        graph.add_edge(prev_node, next_node, len);

        for dir in valid_directions(next, next_pred, grid) {
            queue.push_back((next + dir.into(), next));
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
