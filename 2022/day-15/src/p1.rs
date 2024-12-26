use utility_belt::prelude::*;

use crate::parser::*;

pub fn part1(input: &PuzzleInput) -> String {
    row_exclusion_zone(input, 2000000).to_string()
}

pub fn row_exclusion_zone(input: &PuzzleInput, row: i32) -> usize {
    let mut exclusion_zone = HashSet::new();

    for (sensor, beacon) in input.sensors.iter() {
        let distance = sensor.manhattan_distance(*beacon);
        let y_distance = (sensor.y - row).abs();

        for dx in [-1, 0, 1] {
            for offset in 0..=(distance - y_distance) {
                exclusion_zone.insert(Coordinate::new(sensor.x + dx * offset, row));
            }
        }
    }

    for (_sensor, beacon) in input.sensors.iter() {
        if beacon.y == row {
            exclusion_zone.remove(beacon);
        }
    }

    exclusion_zone.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = indoc! {"
        Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        Sensor at x=9, y=16: closest beacon is at x=10, y=16
        Sensor at x=13, y=2: closest beacon is at x=15, y=3
        Sensor at x=12, y=14: closest beacon is at x=10, y=16
        Sensor at x=10, y=20: closest beacon is at x=10, y=16
        Sensor at x=14, y=17: closest beacon is at x=10, y=16
        Sensor at x=8, y=7: closest beacon is at x=2, y=10
        Sensor at x=2, y=0: closest beacon is at x=2, y=10
        Sensor at x=0, y=11: closest beacon is at x=2, y=10
        Sensor at x=20, y=14: closest beacon is at x=25, y=17
        Sensor at x=17, y=20: closest beacon is at x=21, y=22
        Sensor at x=16, y=7: closest beacon is at x=15, y=3
        Sensor at x=14, y=3: closest beacon is at x=15, y=3
        Sensor at x=20, y=1: closest beacon is at x=15, y=3
    "};

    #[test]
    fn test_part1_example() {
        let input = crate::parser::part1(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(row_exclusion_zone(&input, 10), 26);
    }
}
