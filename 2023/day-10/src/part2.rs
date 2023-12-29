use std::collections::VecDeque;

use crate::{
    part1::{adjacent, find_start_pipes},
    structs::*,
};

use utility_belt::{math::union_find::UnionFind, prelude::*};

pub fn part2(input: &PuzzleInput) -> String {
    let grid = input.grid.clone();

    // Mask out everything that is outside of the loop
    let start = grid.iter().find(|(_, c)| **c == 'S').unwrap().0;
    let start_pipes = find_start_pipes(start, input);
    let loop_mask = loop_mask(&grid, start, start_pipes);

    let clean = Grid2D::from_shape_vec(
        grid.width(),
        grid.height(),
        grid.iter()
            .map(|(coord, c)| if loop_mask[coord] { *c } else { '.' })
            .collect_vec(),
    );

    // Zoom the grid, so that squeezing between pipes is possible
    let loop_mask = loop_mask.zoom(2);
    let mut grid = clean.template_zoom(|c: &char| {
        match c {
            '|' => [
                ['|', '_'], //
                ['|', '_'],
            ],

            '-' => [
                ['-', '-'], //
                ['_', '_'],
            ],

            '.' => [
                ['.', '.'], //
                ['.', '.'],
            ],

            'S' => [
                ['S', 'S'], //
                ['S', 'S'],
            ],

            'L' => [
                ['L', '-'], //
                ['_', '_'],
            ],

            'J' => [
                ['J', '_'], //
                ['_', '_'],
            ],

            '7' => [
                ['7', '_'], //
                ['|', '_'],
            ],

            'F' => [
                ['F', '-'], //
                ['|', '_'],
            ],

            _ => unreachable!(),
        }
    });

    // Add an implicit border around the grid, so that all outside cells are connected
    let grid = BorderedGrid2D::new(1, '.', &mut grid);

    // Find the connected components in the resulting grid
    let mut uf = UnionFind::default();
    let mut indices = HashMap::new();

    grid.iter().for_each(|(coord, _c)| {
        indices.insert(coord, uf.make_set());
    });

    grid.iter().for_each(|(coord, c)| {
        let mut adj = adjacent(*c);

        if *c == '.' || *c == '_' {
            adj = DirectionSet::all();
        }

        for dir in adj.iter() {
            let next = coord.neighbor(dir);

            if let Some(nc) = grid.get(next) {
                if (*c == '.' || *c == '_') && (*nc != '.' && *nc != '_') {
                    continue;
                }

                if (*nc == '.' || *nc == '_') && (*c != '.' && *c != '_') {
                    continue;
                }

                let r = uf.union(indices[&coord], indices[&next]);
                assert!(r.is_ok());
            }
        }
    });

    // Count the number of cells that are inside the loop by finding all
    // cells that are not outside and not part of the loop.
    let outside_idx = uf.find(indices[&Coordinate::new(0, 0)]).unwrap();
    let mut inside_count = 0;

    grid.iter().for_each(|(coord, c)| {
        if *c == '.' && uf.find(indices[&coord]).unwrap() != outside_idx && !loop_mask[coord] {
            inside_count += 1;
        }
    });

    // Divide by 4, because each cell is 2x2
    inside_count /= 4;

    // Return.
    inside_count.to_string()
}

fn loop_mask(grid: &Grid2D<char>, start: Coordinate, start_pipes: [Coordinate; 2]) -> Grid2D<bool> {
    let mut queue = VecDeque::new();

    let mut loop_mask = Grid2D::new(grid.width(), grid.height(), false);
    loop_mask[start] = true;

    queue.push_back(start_pipes[0]);
    queue.push_back(start_pipes[1]);

    while let Some(cur) = queue.pop_front() {
        if loop_mask[cur] {
            continue;
        }

        loop_mask[cur] = true;

        for dir in adjacent(grid[cur]).iter() {
            let next = cur.neighbor(dir);
            queue.push_back(next);
        }
    }

    loop_mask
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::indoc;

    const TEST_INPUT: &str = indoc! {"
        ...........
        .S-------7.
        .|F-----7|.
        .||.....||.
        .||.....||.
        .|L-7.F-J|.
        .|..|.|..|.
        .L--J.L--J.
        ...........
    "};

    const TEST_INPUT2: &str = indoc! {"
        ..........
        .S------7.
        .|F----7|.
        .||....||.
        .||....||.
        .|L-7F-J|.
        .|..||..|.
        .L--JL--J.
        ..........
    "};

    const TEST_INPUT3: &str = indoc! {"
        FF7FSF7F7F7F7F7F---7
        L|LJ||||||||||||F--J
        FL-7LJLJ||||||LJL-77
        F--JF--7||LJLJ7F7FJ-
        L---JF-JLJ.||-FJLJJ7
        |F|F-JF---7F7-L7L|7|
        |FFJF7L7F-JF7|JL---7
        7-L-JL7||F7|L7F-7F7|
        L.L7LFJ|||||FJL7||LJ
        L7JLJL-JLJLJL--JLJ.L
    "};

    #[test]
    fn test_part2() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part2(&input), "4");
        let input = crate::parser::parse(TEST_INPUT2);
        assert_eq!(part2(&input), "4");
        let input = crate::parser::parse(TEST_INPUT3);
        assert_eq!(part2(&input), "10");
    }
}
