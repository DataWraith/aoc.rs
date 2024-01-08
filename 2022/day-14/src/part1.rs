use crate::structs::*;

use utility_belt::prelude::*;

pub fn part1(input: &PuzzleInput) -> String {
    let mut grid: HashSet<Coordinate> = make_grid(input);

    let abyss = input
        .segments
        .iter()
        .flatten()
        .map(|c| c.y() + 1)
        .max()
        .unwrap();

    for count in 0.. {
        if fall(&mut grid, abyss, i32::MAX, Coordinate::new(500, 0)) {
            return count.to_string();
        }
    }

    unreachable!()
}

pub fn make_grid(input: &PuzzleInput) -> HashSet<Coordinate> {
    let mut grid = HashSet::default();

    for segment in input.segments.iter() {
        for (c1, c2) in segment.iter().tuple_windows() {
            for x in c1.x().min(c2.x())..=c2.x().max(c1.x()) {
                grid.insert(Coordinate::new(x, c1.y()));
            }

            for y in c1.y().min(c2.y())..=c2.y().max(c1.y()) {
                grid.insert(Coordinate::new(c1.x(), y));
            }
        }
    }

    grid
}

pub fn fall(
    grid: &mut HashSet<Coordinate>,
    abyss: i32,
    floor: i32,
    coordinate: Coordinate,
) -> bool {
    let mut current = coordinate;

    if grid.contains(&coordinate) {
        return true;
    }

    loop {
        let below = current + Direction::Down.into();

        if below.y() >= abyss {
            return true;
        }

        if below.y() >= floor {
            grid.insert(current);
            break;
        }

        if !grid.contains(&below) {
            current = below;
            continue;
        }

        let below_left = below + Direction::Left.into();

        if !grid.contains(&below_left) {
            current = below_left;
            continue;
        }

        let below_right = below + Direction::Right.into();

        if !grid.contains(&below_right) {
            current = below_right;
            continue;
        }

        grid.insert(current);
        break;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part1(&input), "24");
    }
}
