use crate::structs::*;

use utility_belt::prelude::*;

pub fn part1(input: &PuzzleInput) -> String {
    let visible = calculate_visibility(input);

    return visible.iter().filter(|(_, &v)| v).count().to_string();
}

fn calculate_visibility(input: &PuzzleInput) -> Grid2D<bool> {
    let mut visible = Grid2D::new(input.grid.width(), input.grid.height(), false);
    let mut grid = input.grid.clone();

    for _ in 0..4 {
        for (y, row) in grid.row_iter().enumerate() {
            // Must be smaller than '0', so 0 fits the bill.
            let mut tallest_tree = 0 as char;

            for (x, tree) in row.iter().enumerate() {
                if *tree > tallest_tree {
                    visible[Coordinate::new(x as i32, y as i32)] = true;
                    tallest_tree = *tree;
                }
            }
        }

        // Rotating the grids is cheap, because it just involves the
        // manipulation of the axes (transpose, mirror), we're not actually
        // moving any data around.
        grid.rotate_right();
        visible.rotate_right();
    }

    visible
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part1(&input), "21");
    }
}
