use utility_belt::prelude::*;

use crate::parser::*;

#[tracing::instrument(skip(input))]
pub fn part2(input: &PuzzleInput) -> String {
    let mut trajectories = input.robots.clone();

    'outer: for i in 1.. {
        for robot in trajectories.iter_mut() {
            robot.step();
        }

        // Interestingly, creating a grid from scratch here is faster than
        // reusing a single grid that is updated in place.
        let mut grid = Grid2D::new(101, 103, false);

        for robot in trajectories.iter() {
            // If you think about how the puzzle might have been designed, the
            // initial state of all robots was probably unique and then
            // perturbed by the inverse velocities.
            //
            // So in the initial picture showing the christmas tree, all robots
            // would have been in unique positions. So we discard any state
            // where two robots overlap.
            if grid.get(robot.position) == Some(&true) {
                continue 'outer;
            }

            grid.set(robot.position, true);
        }

        return i.to_string();
    }

    unreachable!()
}
