use utility_belt::prelude::*;

use crate::parser::*;

#[tracing::instrument(skip(input))]
pub fn part2(input: &PuzzleInput) -> String {
    for i in 0..=17050 {
        let mut robot_grid = Grid2D::new(101, 103, '.');

        for robot in input.robots.iter() {
            let mut pos = robot.position + robot.velocity * i;
            let mut final_pos = Coordinate::new(
                pos.x.rem_euclid(robot_grid.width),
                pos.y.rem_euclid(robot_grid.height),
            );
            robot_grid[final_pos] = '#';
        }

        let picture = format!("{}\ni: {}", robot_grid, i);
        if picture.contains("###############################") {
            println!("{}", picture);
            let mut buf = String::new();
            std::io::stdin().read_line(&mut buf).unwrap();
        }
    }

    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = indoc! {"
        TODO
    "};

    #[test]
    fn test_part2_example() {
        let input = crate::parser::part2(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(part2(&input), "TODO");
    }
}
