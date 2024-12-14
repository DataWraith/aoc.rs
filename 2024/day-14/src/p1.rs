use utility_belt::prelude::*;

use crate::parser::*;

#[tracing::instrument(skip(input))]
pub fn part1(input: &PuzzleInput) -> String {
    let mut robot_grid = Grid2D::new(101, 103, 0usize);

    for robot in input.robots.iter() {
        let mut pos = robot.position + robot.velocity * 100;
        let mut final_pos = Coordinate::new(
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

    dbg!(&x_center, &y_center);

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

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = indoc! {"
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
"};

    #[test]
    fn test_part1_example() {
        let input = crate::parser::part1(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(part1(&input), "TODO");
    }
}
