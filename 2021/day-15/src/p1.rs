use pathfinding::directed::dijkstra::dijkstra;
use utility_belt::prelude::*;

use crate::parser::*;

pub fn part1(input: &PuzzleInput) -> String {
    let successors = |p: &Coordinate| {
        p.neighbors()
            .filter(|c| input.grid.contains_coord(*c))
            .map(|c| (c, input.grid[c]))
    };

    let start = Coordinate::new(0, 0);
    let goal = Coordinate::new(
        input.grid.width() as i32 - 1,
        input.grid.height() as i32 - 1,
    );

    dijkstra(&start, successors, |p| *p == goal)
        .unwrap()
        .1
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;

    #[test]
    fn test_part1_example() {
        let input = parser::part1(parser::TEST_INPUT);
        assert_ne!(parser::TEST_INPUT.trim(), "TODO");
        assert_eq!(part1(&input), "40");
    }
}
