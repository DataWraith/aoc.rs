use utility_belt::prelude::*;

use crate::{
    p1::{fold_left_along_column, fold_up_along_row},
    parser::*,
};

pub fn part2(input: &PuzzleInput) -> String {
    let mut grid = input.grid.clone();

    for fold in input.folds.iter() {
        let (axis, value) = fold;

        if *axis {
            grid = fold_up_along_row(grid, *value);
        } else {
            grid = fold_left_along_column(grid, *value);
        }
    }

    let bounding_box = bounding_box(grid.iter().cloned());
    let mut dbg_grid: Grid2D<bool> = Grid2D::new(
        bounding_box.1.x as usize + 1,
        bounding_box.1.y as usize + 1,
        false,
    );

    for coord in grid {
        dbg_grid[coord] = true;
    }

    let grid: BoolGrid2D = dbg_grid.into();
    format!("{}", grid)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;

    #[test]
    fn test_part2_example() {
        let input = parser::part2(parser::TEST_INPUT);
        assert_ne!(parser::TEST_INPUT.trim(), "TODO");
        assert_eq!(part2(&input), "\n#####\n#...#\n#...#\n#...#\n#####\n");
    }
}
