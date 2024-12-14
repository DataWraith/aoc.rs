use utility_belt::prelude::*;

use crate::parser::*;

#[tracing::instrument(skip(input))]
pub fn part2(input: &PuzzleInput) -> String {
    let mut trajectories = input.robots.clone();

    for i in 1.. {
        for robot in trajectories.iter_mut() {
            robot.step();
        }

        let mut grid = Grid2D::new(101, 103, false);

        for robot in trajectories.iter() {
            grid.set(robot.position, true);
        }

        let mut count = 0;

        for robot in trajectories.iter() {
            if grid.get(robot.position.neighbor(Direction::Right)) == Some(&true) {
                count += 1;
            }
        }

        if count >= 200 {
            return i.to_string();
        }
    }

    unreachable!()
}
