use crate::structs::*;

use utility_belt::prelude::*;

pub fn part1(input: &PuzzleInput) -> String {
    let visible = calculate_visibility(input);

    return visible.iter().filter(|(_, &v)| v).count().to_string();
}

fn calculate_visibility(input: &PuzzleInput) -> Grid2D<bool> {
    let mut visible = Grid2D::new(input.grid.width(), input.grid.height(), false);

    for (x, col) in input.grid.col_iter().enumerate() {
        let top_to_bottom_visibility = col.iter().scan('/', |state, &tree| {
            let result = tree > *state;
            *state = std::cmp::max(*state, tree);
            Some(result)
        });

        for (y, v) in top_to_bottom_visibility.enumerate() {
            let c = Coordinate::new(x as i32, y as i32);

            if v {
                visible.set(c, v);
            }
        }

        let bottom_to_top_visibility = col.iter().rev().scan('/', |state, &tree| {
            let result = tree > *state;
            *state = std::cmp::max(*state, tree);
            Some(result)
        });

        for (y, v) in bottom_to_top_visibility.enumerate() {
            let c = Coordinate::new(x as i32, (input.grid.height() - 1 - y) as i32);

            if v {
                visible.set(c, v);
            }
        }
    }

    for (y, row) in input.grid.row_iter().enumerate() {
        let left_to_right_visibility = row.iter().scan('/', |state, &tree| {
            let result = tree > *state;
            *state = std::cmp::max(*state, tree);
            Some(result)
        });

        for (x, v) in left_to_right_visibility.enumerate() {
            let c = Coordinate::new(x as i32, y as i32);

            if v {
                visible.set(c, v);
            }
        }

        let right_to_left_visibility = row.iter().rev().scan('/', |state, &tree| {
            let result = tree > *state;
            *state = std::cmp::max(*state, tree);
            Some(result)
        });

        for (rev_x, v) in right_to_left_visibility.enumerate() {
            let c = Coordinate::new((input.grid.width() - 1 - rev_x) as i32, y as i32);

            if v {
                visible.set(c, v);
            }
        }
    }

    visible
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part1(&input), "21");
    }
}
