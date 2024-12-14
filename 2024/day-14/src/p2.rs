use utility_belt::prelude::*;

use crate::parser::*;

#[tracing::instrument(skip(input))]
pub fn part2(input: &PuzzleInput) -> String {
    for i in 0.. {
        let mut robot_grid = Grid2D::new(101, 103, '.');

        for robot in input.robots.iter() {
            let pos = robot.position + robot.velocity * i;

            let final_pos = Coordinate::new(
                pos.x.rem_euclid(robot_grid.width),
                pos.y.rem_euclid(robot_grid.height),
            );

            robot_grid[final_pos] = '#';
        }

        let picture = format!("{}\ni: {}", robot_grid, i);

        if picture.contains("###############################") {
            return i.to_string();
        }
    }

    unreachable!()
}
