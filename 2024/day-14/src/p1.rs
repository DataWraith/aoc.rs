use utility_belt::prelude::*;

use crate::parser::*;

#[tracing::instrument(skip(input))]
pub fn part1(input: &PuzzleInput) -> String {
    let mut robot_grid = Grid2D::new(101, 103, 0usize);

    for robot in input.robots.iter() {
        let pos = robot.position + robot.velocity * 100;

        let final_pos = Coordinate::new(
            pos.x.rem_euclid(robot_grid.width),
            pos.y.rem_euclid(robot_grid.height),
        );

        robot_grid[final_pos] += 1;
    }

    let counts = count_robots(&robot_grid);
    let safety_factor = counts[0] * counts[1] * counts[2] * counts[3];

    safety_factor.to_string()
}

pub fn count_robots(grid: &Grid2D<usize>) -> [usize; 4] {
    let x_center = grid.width / 2;
    let y_center = grid.height / 2;
    let mut counts = [0; 4];

    for x in 0..x_center {
        for y in 0..y_center {
            counts[0] += grid[(x, y).into()];
        }

        for y in (y_center + 1)..grid.height {
            counts[1] += grid[(x, y).into()];
        }
    }

    for x in (x_center + 1)..grid.width {
        for y in 0..y_center {
            counts[2] += grid[(x, y).into()];
        }

        for y in (y_center + 1)..grid.height {
            counts[3] += grid[(x, y).into()];
        }
    }

    counts
}
