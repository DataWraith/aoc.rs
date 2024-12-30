use utility_belt::prelude::*;

use crate::{
    p1::{blizzard_step, end_coord, make_blizzard_grid, start_coord},
    parser::*,
};

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct State {
    position: Coordinate,
    goal: Coordinate,
    goals_met: u8,
    time: usize,
}

pub fn part2(input: &PuzzleInput) -> String {
    let (cycle_length, _, cycle_start) =
        pathfinding::directed::cycle_detection::brent(make_blizzard_grid(input), |grid| {
            blizzard_step(&grid)
        });

    assert_eq!(cycle_start, 0);

    let blizzard_grids = std::iter::successors(Some(make_blizzard_grid(input)), |grid| {
        Some(blizzard_step(grid))
    })
    .take(cycle_length)
    .map(|grid| grid.map(|ds| !ds.is_empty()))
    .map(|grid| grid.into())
    .collect::<Vec<BoolGrid2D>>();

    let start = State {
        position: start_coord(input),
        goal: end_coord(input),
        goals_met: 0,
        time: 1,
    };

    let start_coord = start_coord(input);
    let end_coord = end_coord(input);

    let successors = |state: &State| {
        let blizzard_grid = &blizzard_grids[state.time % blizzard_grids.len()];

        let mut successors = Vec::new();

        for n in state.position.neighbors() {
            if n != state.goal
                && (n.x == 0
                    || n.x == input.grid.width() as i32 - 1
                    || n.y <= 0 // Can walk off the top edge and bottom edge.
                    || n.y >= input.grid.height() as i32 - 1)
            {
                continue;
            }

            if blizzard_grid[n.into()] {
                continue;
            }

            if n == state.goal {
                let new_goal = if n == start_coord {
                    end_coord
                } else {
                    start_coord
                };

                let new_state = State {
                    position: n,
                    time: state.time + 1,
                    goal: new_goal,
                    goals_met: state.goals_met + 1,
                };

                successors.push((new_state, 1));
            } else {
                successors.push((
                    State {
                        position: n,
                        time: state.time + 1,
                        goal: state.goal,
                        goals_met: state.goals_met,
                    },
                    1,
                ));
            }
        }

        if !blizzard_grid[state.position.into()] {
            successors.push((
                State {
                    position: state.position,
                    time: state.time + 1,
                    goal: state.goal,
                    goals_met: state.goals_met,
                },
                1,
            ));
        }

        successors
    };

    let path =
        pathfinding::directed::dijkstra::dijkstra(&start, successors, |state| state.goals_met == 3);

    (path.unwrap().1).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = indoc! {"
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
