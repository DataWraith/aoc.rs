use crate::structs::*;

use utility_belt::{math::polygons::polygon_area, prelude::*};

pub fn part1(input: &PuzzleInput) -> String {
    let area = polygon_area(&coordinates(input));
    let perimeter = perimeter(input);

    (area as usize + perimeter / 2 + 1).to_string()
}

pub fn perimeter(input: &PuzzleInput) -> usize {
    input.plan.iter().map(|x| x.distance).sum::<usize>()
}

pub fn coordinates(input: &PuzzleInput) -> Vec<(isize, isize)> {
    let mut result = Vec::new();
    let mut x = 0;
    let mut y = 0;

    result.push((x, y));

    for edge in input.plan.iter() {
        let distance = edge.distance as isize;

        match edge.direction {
            Direction::Right => {
                x += distance;
            }
            Direction::Down => {
                y += distance;
            }
            Direction::Left => {
                x -= distance;
            }
            Direction::Up => {
                y -= distance;
            }
        }

        result.push((x, y));
    }

    result
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
    fn test_part1() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part1(&input), "62");
    }
}
