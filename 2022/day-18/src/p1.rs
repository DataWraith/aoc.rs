use utility_belt::prelude::*;

use crate::parser::*;

pub fn part1(input: &PuzzleInput) -> String {
    let mut total_sides = 0;

    for (x, y, z) in input.cubes.iter() {
        let mut sides = 6;

        for (dx, dy, dz) in [
            (1, 0, 0),
            (-1, 0, 0),
            (0, 1, 0),
            (0, -1, 0),
            (0, 0, 1),
            (0, 0, -1),
        ] {
            if input.cubes.contains(&(x + dx, y + dy, z + dz)) {
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
    use utility_belt::prelude::*;

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
