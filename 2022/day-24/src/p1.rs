use pathfinding;
use utility_belt::prelude::*;

use crate::parser::*;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct State {
    // The current position of the expedition
    pub position: Coordinate,
    // The distance a blizzard would have to travel to get to us
    pub blizzard_index: usize,
    // The index of the current goal
    pub goal_index: usize,
}

pub fn part1(input: &PuzzleInput) -> String {
    // Find the cycle length of the blizzard grid and make a map of the blizzard
    // locations and orientations.
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
        |state| state.position == end_coord,
    );

    // Subtract one to account for the start state being included in the path.
    (path.unwrap().len() - 1).to_string()
}

// The horizontal and vertical blizzards cycle every (width - 2) and (height - 2)
// steps respectively. For both cycles to align, we need to find the least common
// multiple of the two numbers.
pub fn blizzard_cycle_length(input: &PuzzleInput) -> usize {
    let width = input.grid.width() - 2;
    let height = input.grid.height() - 2;

    lcm(width, height)
}

// Given a state, returns all the states we can move to
pub fn successor_states(
    blizzard_grid: &Grid2D<Option<Direction>>,
    cycle_length: usize,
    current_state: &State,
    goals: &[Coordinate; 3],
) -> Vec<State> {
    let mut successors = Vec::new();

    // Chain the four neighbors with the current position to allow for staying
    // in place.
    'outer: for n in current_state
        .position
        .neighbors()
        .chain([current_state.position])
    {
        // Check if we've reached the start or goal
        if goals.contains(&n) {
            // Is it our current goal?
            if n == goals[current_state.goal_index] {
                let new_state = State {
                    position: n,
                    blizzard_index: (current_state.blizzard_index + 1) % cycle_length,
                    goal_index: current_state.goal_index + 1,
                };

                successors.push(new_state);
                continue;
            }

            // Nope. But we're safe at the start and goal, so we can move or stay there.
            successors.push(State {
                position: n,
                blizzard_index: (current_state.blizzard_index + 1) % cycle_length,
                ..current_state.clone()
            });

            continue;
        }

        // Check if we would move out of bounds / into a wall.
        if !blizzard_grid.contains(n) {
            continue;
        }

        // Check if we're about to walk into a blizzard.
        for direction in Direction::cardinal() {
            // Figure out how far the blizzard would have to travel from the
            // start to get to us.
            let distance = current_state.blizzard_index;

            // Move `distance` steps in the assumed direction of the blizzard.
            let blizzard_coord = n + direction * distance as i32;

            // Wrap around the grid if necessary
            let blizzard_coord = blizzard_coord % blizzard_grid.dims();

            // If there is a blizzard at `blizzard_coord` at T=0, and it's
            // moving towards us, we can't move to `n` because the blizzard
            // would hit us at the current timestep.
            if blizzard_grid.get(blizzard_coord) == Some(&Some(direction.opposite())) {
                continue 'outer;
            }
        }

        // Otherwise we can move to that position
        successors.push(State {
            position: n,
            blizzard_index: (current_state.blizzard_index + 1) % cycle_length,
            ..current_state.clone()
        });
    }

    successors
}

pub fn find_start_coord(input: &PuzzleInput) -> Coordinate {
    for x in 0..input.grid.width() {
        if input.grid.get((x as i32, 0i32).into()).unwrap() == &'.' {
            return Coordinate::new(x as i32, 0);
        }
    }

    unreachable!()
}

pub fn find_end_coord(input: &PuzzleInput) -> Coordinate {
    for x in (0..input.grid.width()).rev() {
        if input
            .grid
            .get((x as i32, input.grid.height() as i32 - 1).into())
            .unwrap()
            == &'.'
        {
            return Coordinate::new(x as i32, input.grid.height() as i32 - 1);
        }
    }

    unreachable!()
}

// Given the puzzle input, make the blizzard grid. The blizzard grid has all the
// blizzards at T=0, and we also cut off the outer ring of walls for convenience.
pub fn make_blizzard_grid(input: &PuzzleInput) -> Grid2D<Option<Direction>> {
    let mut grid = Grid2D::new(input.grid.width() - 2, input.grid.height() - 2, None);

    for y in 1..(input.grid.height() - 1) {
        for x in 1..(input.grid.width() - 1) {
            let cell = input.grid.get((x as i32, y as i32).into()).unwrap();

            match cell {
                '^' => grid[(x as i32 - 1, y as i32 - 1).into()] = Some(Direction::Up),
                'v' => grid[(x as i32 - 1, y as i32 - 1).into()] = Some(Direction::Down),
                '<' => grid[(x as i32 - 1, y as i32 - 1).into()] = Some(Direction::Left),
                '>' => grid[(x as i32 - 1, y as i32 - 1).into()] = Some(Direction::Right),
                _ => {}
            }
        }
    }

    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = indoc! {"
        #.######
        #>>.<^<#
        #.<..<<#
        #>v.><>#
        #<^v^^>#
        ######.#
    "};

    #[test]
    fn test_part1_example() {
        let input = crate::parser::part1(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(part1(&input), "18");
    }
}
