use utility_belt::prelude::*;

use crate::parser::*;

pub fn part1(input: &PuzzleInput) -> String {
    let x_center = 101 / 2;
    let y_center = 103 / 2;
    let map_size = Coordinate::new(101, 103);

    let mut counts = [0; 4];

    for robot in input.robots.iter() {
        let pos = robot.position + robot.velocity * 100;
        let pos = pos % map_size;

        if pos.x < x_center && pos.y < y_center {
            counts[0] += 1;
        } else if pos.x < x_center && pos.y > y_center {
            counts[1] += 1;
        } else if pos.x > x_center && pos.y < y_center {
            counts[2] += 1;
        } else if pos.x > x_center && pos.y > y_center {
            counts[3] += 1;
        }
    }

    counts.iter().product::<usize>().to_string()
}
