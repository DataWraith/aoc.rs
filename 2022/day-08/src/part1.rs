use crate::structs::*;

use utility_belt::prelude::*;

pub fn part1(input: &PuzzleInput) -> String {
    let visible = calculate_visibility(input);

    return visible.iter().filter(|(_, &v)| v).count().to_string();
}

fn calculate_visibility(input: &PuzzleInput) -> Grid2D<bool> {
    let mut visible = Grid2D::new(input.grid.width(), input.grid.height(), false);

    let visibility_fn = |state: &mut char, tree: &char| {
        let result = *tree > *state;
        *state = (*state).max(*tree);
        Some(result)
    };

    for (x, col) in input.grid.col_iter().enumerate() {
        let top_to_bottom_visibility = col.iter().scan('/', visibility_fn);
        let bottom_to_top_visibility = col.iter().rev().scan('/', visibility_fn);

        for (y, v) in top_to_bottom_visibility.enumerate() {
            let c = Coordinate::new(x as i32, y as i32);

            if v {
                visible.set(c, true);
            }
        }

        for (y, v) in bottom_to_top_visibility.enumerate() {
            let c = Coordinate::new(x as i32, input.grid.height() as i32 - 1 - y as i32);

            if v {
                visible.set(c, true);
            }
        }
    }

    for (y, row) in input.grid.row_iter().enumerate() {
        let left_to_right_visibility = row.iter().scan('/', visibility_fn);
        let right_to_left_visibility = row.iter().rev().scan('/', visibility_fn);

        for (x, v) in left_to_right_visibility.enumerate() {
            let c = Coordinate::new(x as i32, y as i32);

            if v {
                visible.set(c, true);
            }
        }

        for (x, v) in right_to_left_visibility.enumerate() {
            let c = Coordinate::new(input.grid.width() as i32 - 1 - x as i32, y as i32);

            if v {
                visible.set(c, true);
            }
        }
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
