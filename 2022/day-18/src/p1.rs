use utility_belt::prelude::*;

use crate::parser::*;

pub const NEIGHBOR_OFFSETS: [IVec3; 6] = [
    IVec3::X,
    IVec3::NEG_X,
    IVec3::Y,
    IVec3::NEG_Y,
    IVec3::Z,
    IVec3::NEG_Z,
];

pub fn part1(input: &PuzzleInput) -> String {
    let mut total_sides = 0;

    for cube in input.cubes.iter() {
        let mut sides = 6;

        for offset in NEIGHBOR_OFFSETS {
            let cur = cube + offset;

            if input.cubes.contains(&cur) {
                sides -= 1;
            }
        }

        total_sides += sides;
    }

    total_sides.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = indoc! {"
        2,2,2
        1,2,2
        3,2,2
        2,1,2
        2,3,2
        2,2,1
        2,2,3
        2,2,4
        2,2,6
        1,2,5
        3,2,5
        2,1,5
        2,3,5
    "};

    #[test]
    fn test_part1_example() {
        let input = crate::parser::part1(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(part1(&input), "64");
    }
}
