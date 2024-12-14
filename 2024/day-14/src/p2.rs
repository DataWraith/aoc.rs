use utility_belt::prelude::*;

use crate::parser::*;

#[tracing::instrument(skip(input))]
pub fn part2(input: &PuzzleInput) -> String {
    for i in 0..=6446 {
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
            println!("{}", picture);

            // Wait for user to press enter, in case the picture does not show a
            // christmas tree
            let mut buf = String::new();
            std::io::stdin().read_line(&mut buf).unwrap();
        }
    }

    "".to_string()
}
