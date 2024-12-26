use utility_belt::prelude::*;

use crate::{p1::row_exclusion_zone, parser::*};

pub fn part2(input: &PuzzleInput) -> String {
    for y in 0..=4000000 {
        let row_exclusion = row_exclusion_zone(input, y);

        let mut x = 0;

        for (lo, hi) in row_exclusion.iter() {
            if x < *lo {
                let coord = Coordinate::new(*lo as i32 - 1, y as i32);
                return tuning_frequency(coord).to_string();
            } else {
                x = (*hi + 1).max(x);
            }
        }
    }

    unreachable!()
}

fn tuning_frequency(coord: Coordinate) -> u64 {
    coord.x as u64 * 4000000 + coord.y as u64
}
