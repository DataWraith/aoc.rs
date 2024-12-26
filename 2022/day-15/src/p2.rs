use utility_belt::prelude::*;

use crate::parser::*;

pub fn part2(input: &PuzzleInput) -> String {
    let max_coord = Coordinate::new(4000001, 4000001);
    let beacon = find_beacon_position(input, max_coord);
    let tuning_frequency = tuning_frequency(beacon);

    tuning_frequency.to_string()
}

fn tuning_frequency(coord: Coordinate) -> u64 {
    coord.x as u64 * 4000000 + coord.y as u64
}

pub fn find_beacon_position(input: &PuzzleInput, max_coord: Coordinate) -> Coordinate {
    let mut potential_beacon_positions = HashSet::new();

    for (sensor, beacon) in input.sensors.iter() {
        let distance = sensor.manhattan_distance(*beacon);

        for dist_x in (-distance - 1)..=(distance + 1) {
            let dist_y = (distance + 1) - dist_x.abs();

            let x = sensor.x + dist_x;
            let y = sensor.y + dist_y;
            let y2 = sensor.y - dist_y;

            if x >= 0 && x <= max_coord.x && y >= 0 && y <= max_coord.y {
                potential_beacon_positions.insert(Coordinate::new(x, y));
            }

            if x >= 0 && x <= max_coord.x && y2 >= 0 && y2 <= max_coord.y {
                potential_beacon_positions.insert(Coordinate::new(x, y2));
            }
        }
    }

    for (sensor, beacon) in input.sensors.iter() {
        let distance = sensor.manhattan_distance(*beacon);

        potential_beacon_positions.retain(|coord| {
            let distance_to_sensor = sensor.manhattan_distance(*coord);
            distance_to_sensor > distance
        });
    }

    assert_eq!(potential_beacon_positions.len(), 1);

    potential_beacon_positions.into_iter().next().unwrap()
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
    fn test_part2_example() {
        let input = crate::parser::part2(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(
            tuning_frequency(find_beacon_position(&input, Coordinate::new(21, 21))),
            56000011
        );
    }
}
