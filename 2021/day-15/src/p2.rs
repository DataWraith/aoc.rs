use pathfinding::directed::dijkstra::dijkstra;
use utility_belt::prelude::*;

use crate::parser::*;

pub fn part2(input: &PuzzleInput) -> String {
    let grid = enlarge_grid(&input.grid);

    let successors = |p: &Coordinate| {
        p.neighbors()
            .filter(|c| grid.contains_coord(*c))
            .map(|c| (c, grid[c]))
    };

    dijkstra(&Coordinate::new(0, 0), successors, |p| {
        *p == Coordinate::new(grid.width() as i32 - 1, grid.height() as i32 - 1)
    })
    .unwrap()
    .1
    .to_string()
}

fn enlarge_grid(input_grid: &Grid2D<u32>) -> Grid2D<u32> {
    let mut grid = input_grid.replicate(5, 5);

    for x in 0..(5 * input_grid.width()) {
        for y in 0..(5 * input_grid.height()) {
            let coord = Coordinate::new(x as i32, y as i32);
            let value = grid[coord];
            let increase = x / input_grid.width() + y / input_grid.height();
            grid[coord] = (value + increase as u32 - 1) % 9 + 1;
        }
    }

    grid
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;
    use utility_belt::prelude::*;

    #[test]
    fn test_part2_example() {
        let input = parser::part2(parser::TEST_INPUT);
        assert_ne!(parser::TEST_INPUT.trim(), "TODO");
        assert_eq!(part2(&input), "315");
    }
}
