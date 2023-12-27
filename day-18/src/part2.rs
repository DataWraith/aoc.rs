use crate::{
    part1::{coordinates, perimeter},
    structs::*,
};

use utility_belt::{math::polygons::polygon_area, prelude::*};

pub fn part2(input: &PuzzleInput) -> String {
    let input = convert_input(input);
    let area = polygon_area(&coordinates(&input));
    let perimeter = perimeter(&input);

    (area as usize + perimeter / 2 + 1).to_string()
}

fn convert_input(input: &PuzzleInput) -> PuzzleInput {
    let mut result = Vec::new();

    for edge in input.plan.iter() {
        let r = edge.color.0;
        let g = edge.color.1;
        let b = edge.color.2;

        let direction = match b & 0xf {
            0x0 => Direction::Right,
            0x1 => Direction::Down,
            0x2 => Direction::Left,
            0x3 => Direction::Up,
            _ => panic!("Invalid direction"),
        };

        let mut distance = (b >> 4) as u32;
        distance += (g as u32) << 4;
        distance += (r as u32) << 12;

        result.push(PlanEdge {
            direction,
            distance: distance as usize,
            color: edge.color,
        });
    }

    PuzzleInput { plan: result }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::indoc;

    const TEST_INPUT: &str = indoc! {"
        R 6 (#70c710)
        D 5 (#0dc571)
        L 2 (#5713f0)
        D 2 (#d2c081)
        R 2 (#59c680)
        D 2 (#411b91)
        L 5 (#8ceee2)
        U 2 (#caa173)
        L 1 (#1b58a2)
        U 2 (#caa171)
        R 2 (#7807d2)
        U 3 (#a77fa3)
        L 2 (#015232)
        U 2 (#7a21e3)
    "};

    #[test]
    fn test_part2() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part2(&input), "952408144115");
    }
}
