use rangetools::{BoundedSet, Rangetools};

use crate::parser::*;

pub fn part1(input: &PuzzleInput) -> String {
    let mut range = row_exclusion_zone(input, 2000000);

    for (_, beacon) in input.sensors.iter() {
        if beacon.y == 2000000 {
            let beacon_range = (beacon.x as i32)..=(beacon.x as i32);
            let beacon_complement = beacon_range.complement();
            range = range.intersection(beacon_complement);
        }
    }

    range.into_iter().count().to_string()
}

pub fn row_exclusion_zone(input: &PuzzleInput, row: i32) -> BoundedSet<i32> {
    let mut intervals = BoundedSet::empty();

    for (sensor, beacon) in input.sensors.iter() {
        // First, calculate the distance to the beacon
        let distance = sensor.manhattan_distance(*beacon);

        // When the distance to the row is `y`, the distance along the row maxes out at `distance - y`
        let y_distance = (sensor.y - row).abs();
        let offset = distance - y_distance;

        // If the offset is negative, then the y_distance is too large, and the
        // sensor does not affect the row
        if offset < 0 {
            continue;
        }

        intervals = intervals.union((sensor.x - offset)..=(sensor.x + offset));
    }

    intervals
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = utility_belt::prelude::indoc! {"
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
        let expected = BoundedSet::empty();
        let expected = expected.union((-2..=24).into_iter());

        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(row_exclusion_zone(&input, 10), expected);
    }
}
