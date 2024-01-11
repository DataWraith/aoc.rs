use crate::{
    part1::{fall, make_grid},
    structs::*,
};

use utility_belt::prelude::*;

pub fn part2(input: &PuzzleInput) -> String {
    let mut grid: HashSet<Coordinate> = make_grid(input);

    let floor = input
        .segments
        .iter()
        .flatten()
        .map(|c| c.y + 2)
        .max()
        .unwrap();

    for count in 0.. {
        if fall(&mut grid, i32::MAX, floor, Coordinate::new(500, 0)) {
            return count.to_string();
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part2() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part2(&input), "93");
    }
}
