use utility_belt::prelude::*;

use crate::{
    p1::{
        blizzard_cycle_length, find_end_coord, find_start_coord, make_blizzard_grid,
        successor_states, State,
    },
    parser::*,
};

pub fn part2(input: &PuzzleInput) -> String {
    // Find the cycle length of the blizzard grid and make a BoolGrid2D for each step.
    // The grid tells us whether it is safe to be in a cell at a given time.
    let cycle_length = blizzard_cycle_length(input);
    let blizzard_grid = make_blizzard_grid(input);

    // Find the start and end coordinates and define our goals
    let start_coord = find_start_coord(input);
    let end_coord = find_end_coord(input);

    // Account for the fact that we cut off the outer ring of walls.
    let start_coord = start_coord - Coordinate::new(1, 1);
    let end_coord = end_coord - Coordinate::new(1, 1);

    // Define our goals
    let goals = [end_coord, start_coord, end_coord];

    // And then, it's just Dijkstra's
    let start = State {
        position: start_coord,
        blizzard_index: 1,
        goal_index: 0,
    };

    let path = pathfinding::directed::bfs::bfs(
        &start,
        |state| successor_states(&blizzard_grid, cycle_length, state, &goals),
        |state| state.goal_index == 3,
    );

    // Subtract one to account for the start state being included in the path.
    (path.unwrap().len() - 1).to_string()
}

#[cfg(test)]
mod tests {

    const TEST_INPUT: &str = utility_belt::prelude::indoc! {"
        #.######
        #>>.<^<#
        #.<..<<#
        #>v.><>#
        #<^v^^>#
        ######.#
    "};

    #[test]
    fn test_part2_example() {
        let input = crate::parser::part2(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(crate::p2::part2(&input), "54");
    }
}
