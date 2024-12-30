use pathfinding;
use utility_belt::prelude::*;

use crate::parser::*;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct State {
    // The current position of the expedition
    pub position: Coordinate,
    // The index of the applicable blizzard grid
    pub blizzard_index: usize,
    // The index of the current goal
    pub goal_index: usize,
}

pub fn part1(input: &PuzzleInput) -> String {
    // Find the cycle length of the blizzard grid and make a BoolGrid2D for each step.
    // The grid tells us whether it is safe to be in a cell at a given time.
    let cycle_length = blizzard_cycle_length(input);
    let blizzard_grids = make_blizzard_grids(input, cycle_length);

    // Find the start and end coordinates and define our goals
    let start_coord = find_start_coord(input);
    let end_coord = find_end_coord(input);
    let goals = [end_coord, start_coord, end_coord];

    // And then, it's just Dijkstra's
    let start = State {
        position: start_coord,
        blizzard_index: 1,
        goal_index: 0,
    };

    let path = pathfinding::directed::bfs::bfs(
        &start,
        |state| successor_states(&blizzard_grids, state, &goals),
        |state| state.position == end_coord,
    );

    // Subtract 1 because we don't count the start state
    (path.unwrap().len() - 1).to_string()
}

pub fn blizzard_cycle_length(input: &PuzzleInput) -> usize {
    let width = input.grid.width() - 2;
    let height = input.grid.height() - 2;

    lcm(width, height)
}

pub fn make_blizzard_grids(input: &PuzzleInput, cycle_length: usize) -> Vec<BoolGrid2D> {
    std::iter::successors(Some(make_blizzard_grid(input)), |grid| {
        Some(blizzard_step(grid))
    })
    .take(cycle_length)
    .map(|grid| grid.map(|ds| !ds.is_empty()))
    .map(|grid| grid.into())
    .collect::<Vec<BoolGrid2D>>()
}

// Given a state, returns all the states we can move to
pub fn successor_states(
    blizzard_grids: &[BoolGrid2D],
    current_state: &State,
    goals: &[Coordinate; 3],
) -> Vec<State> {
    let blizzard_grid = &blizzard_grids[current_state.blizzard_index % blizzard_grids.len()];

    let mut successors = Vec::new();

    for n in current_state.position.neighbors() {
        // Check if we're about to walk into a wall
        if n != goals[current_state.goal_index]
            && (n.x == 0
                    || n.x == blizzard_grid.width() as i32 - 1
                    // Can walk off the top edge and bottom edge, so this needs to be <= and >=
                    || n.y <= 0
                    || n.y >= blizzard_grid.height() as i32 - 1)
        {
            continue;
        }

        // Check if we're about to walk into a blizzard
        if blizzard_grid[n] {
            continue;
        }

        // Check if we've reached a goal
        if n == goals[current_state.goal_index] {
            let new_state = State {
                position: n,
                blizzard_index: current_state.blizzard_index + 1,
                goal_index: current_state.goal_index + 1,
            };

            successors.push(new_state);
            continue;
        }

        // Otherwise we can move to that position
        successors.push(State {
            position: n,
            blizzard_index: current_state.blizzard_index + 1,
            ..current_state.clone()
        });
    }

    // If we would not be hit by a blizzard, we can stay in the same position
    if !blizzard_grid[current_state.position] {
        successors.push(State {
            position: current_state.position,
            blizzard_index: current_state.blizzard_index + 1,
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

// Simulate one minute of blizzard movement
pub fn blizzard_step(grid: &Grid2D<DirectionSet>) -> Grid2D<DirectionSet> {
    let mut new_grid = Grid2D::new(grid.width(), grid.height(), DirectionSet::empty());

    for y in 1..(grid.height() - 1) {
        for x in 1..(grid.width() - 1) {
            let cell = grid[(x as i32, y as i32).into()];

            for direction in cell.iter() {
                let mut new_coord = Coordinate::new(x as i32, y as i32) + direction;

                // Wrap around the grid if the blizzard hits a wall.
                //
                // Thankfully, there are no blizzards that can move off the edge
                // of the grid through the start and goal locations.
                if new_coord.x == 0
                    || new_coord.x == grid.width() as i32 - 1
                    || new_coord.y == 0
                    || new_coord.y == grid.height() as i32 - 1
                {
                    new_coord += 2 * direction;
                    new_coord %= grid.dims();
                }

                new_grid[new_coord].insert(direction);
            }
        }
    }

    new_grid
}

// Given the puzzle input, make the initial blizzard grid.
//
// Since blizzards can overlap, using a single direction is inadequate, so we
// use a DirectionSet.
pub fn make_blizzard_grid(input: &PuzzleInput) -> Grid2D<DirectionSet> {
    let mut grid = Grid2D::new(
        input.grid.width(),
        input.grid.height(),
        DirectionSet::empty(),
    );

    for y in 0..input.grid.height() {
        for x in 0..input.grid.width() {
            let cell = input.grid.get((x as i32, y as i32).into()).unwrap();

            match cell {
                '^' => grid[(x as i32, y as i32).into()] = Direction::Up.into(),
                'v' => grid[(x as i32, y as i32).into()] = Direction::Down.into(),
                '<' => grid[(x as i32, y as i32).into()] = Direction::Left.into(),
                '>' => grid[(x as i32, y as i32).into()] = Direction::Right.into(),
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
