use crate::{part1::make_grid, structs::*};

use utility_belt::prelude::*;

pub fn part2(input: &PuzzleInput) -> String {
    let mut grid: HashSet<Coordinate> = make_grid(input);

    let floor = input
        .segments
        .iter()
        .flatten()
        .map(|c| c.y() + 2)
        .max()
        .unwrap();

    for count in 0.. {
        if fall2(&mut grid, floor, Coordinate::new(500, 0)) {
            return count.to_string();
        }
    }

    unreachable!()
}

pub fn fall2(grid: &mut HashSet<Coordinate>, floor: i32, coordinate: Coordinate) -> bool {
    let mut current = coordinate;

    if grid.contains(&coordinate) {
        return true;
    }

    loop {
        let below = current + Direction::Down.into();

        if below.y() >= floor {
            grid.insert(current);
            break;
        }

        if grid.get(&below).is_none() {
            current = below;
            continue;
        }

        let below_left = below + Direction::Left.into();

        if grid.get(&below_left).is_none() {
            current = below_left;
            continue;
        }

        let below_right = below + Direction::Right.into();

        if grid.get(&below_right).is_none() {
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
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part2() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part2(&input), "93");
    }
}
