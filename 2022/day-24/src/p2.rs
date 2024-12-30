use crate::{
    p1::{
        blizzard_cycle_length, find_end_coord, find_start_coord, make_blizzard_grids,
        successor_states, State,
    },
    parser::*,
};

pub fn part2(input: &PuzzleInput) -> String {
    let cycle_length = blizzard_cycle_length(input);
    let blizzard_grids = make_blizzard_grids(input, cycle_length);

    let start_coord = find_start_coord(input);
    let end_coord = find_end_coord(input);
    let goals = [end_coord, start_coord, end_coord];

    let start = State {
        position: start_coord,
        blizzard_index: 1,
        goal_index: 0,
    };

    let path = pathfinding::directed::dijkstra::dijkstra(
        &start,
        |state| successor_states(&blizzard_grids, state, &goals),
        |state| state.goal_index == 3,
    );

    (path.unwrap().1).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(part2(&input), "54");
    }
}
