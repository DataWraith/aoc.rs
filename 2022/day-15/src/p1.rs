use std::collections::BTreeSet;

use crate::parser::*;

pub fn part1(input: &PuzzleInput) -> String {
    let ranges = row_exclusion_zone(input, 2000000);
    let mut beacons = BTreeSet::new();

    'outer: for (_, beacon) in input.sensors.iter() {
        for (start, end) in ranges.iter() {
            if beacon.y == 2000000 && *start <= beacon.x as isize && beacon.x as isize <= *end {
                beacons.insert(beacon.x);
                continue 'outer;
            }
        }
    }

    let mut range_count = 0;

    for (start, end) in ranges.into_iter() {
        range_count += end - start + 1;
    }

    (range_count - beacons.len() as isize).to_string()
}

pub fn row_exclusion_zone(input: &PuzzleInput, row: i32) -> Vec<(isize, isize)> {
    let mut intervals = Vec::new();

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

        intervals.push((sensor.x - offset, sensor.x + offset));
    }

    intervals.sort();

    merge_intervals(intervals)
}

pub fn merge_intervals(intervals: Vec<(i32, i32)>) -> Vec<(isize, isize)> {
    let mut q = Vec::new();

    for (lo, hi) in intervals {
        let lo = lo as isize;
        let hi = hi as isize;

        if q.is_empty() {
            q.push((lo, hi));
            continue;
        }

        let (ql, qh) = q.pop().unwrap();

        if lo > qh + 1 {
            q.push((lo, hi));
            continue;
        }

        q.push((ql, hi.max(qh)));
    }
    q
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
        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(row_exclusion_zone(&input, 10), vec![(-2, 24)]);
    }
}
