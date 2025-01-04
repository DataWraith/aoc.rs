use utility_belt::prelude::*;

use crate::parser::*;

pub fn part1(input: &PuzzleInput) -> String {
    let mut grid = input.grid.clone();

    let fold = input.folds.first().unwrap();
    let (axis, value) = fold;

    if *axis {
        grid = fold_up_along_row(grid, *value);
    } else {
        grid = fold_left_along_column(grid, *value);
    }

    grid.len().to_string()
}

pub fn fold_up_along_row(grid: HashSet<Coordinate>, row: i32) -> HashSet<Coordinate> {
    let mut new_grid = HashSet::new();

    for coord in grid {
        if coord.y < row {
            new_grid.insert(coord);
        } else {
            new_grid.insert(Coordinate::new(coord.x, coord.y - 2 * (coord.y - row)));
        }
    }

    new_grid
}

pub fn fold_left_along_column(grid: HashSet<Coordinate>, column: i32) -> HashSet<Coordinate> {
    let mut new_grid = HashSet::new();

    for coord in grid {
        if coord.x < column {
            new_grid.insert(coord);
        } else {
            new_grid.insert(Coordinate::new(coord.x - 2 * (coord.x - column), coord.y));
        }
    }

    new_grid
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;

    #[test]
    fn test_part1_example() {
        let input = parser::part1(parser::TEST_INPUT);
        assert_ne!(parser::TEST_INPUT.trim(), "TODO");
        assert_eq!(part1(&input), "17");
    }
}
