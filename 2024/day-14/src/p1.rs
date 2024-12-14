use utility_belt::prelude::*;

use crate::parser::*;

#[tracing::instrument(skip(input))]
pub fn part1(input: &PuzzleInput) -> String {
    let x_center = 101 / 2;
    let y_center = 103 / 2;
    let mut counts = [0; 4];

    for robot in input.robots.iter() {
        let pos = robot.position + robot.velocity * 100;

        let final_pos = Coordinate::new(pos.x.rem_euclid(101), pos.y.rem_euclid(103));

        if final_pos.x < x_center && final_pos.y < y_center {
            counts[0] += 1;
        } else if final_pos.x < x_center && final_pos.y > y_center {
            counts[1] += 1;
        } else if final_pos.x > x_center && final_pos.y < y_center {
            counts[2] += 1;
        } else if final_pos.x > x_center && final_pos.y > y_center {
            counts[3] += 1;
        }
    }

    counts.iter().product::<usize>().to_string()
}
