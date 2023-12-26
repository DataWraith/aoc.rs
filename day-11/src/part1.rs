use crate::structs::*;

use utility_belt::{math::cumsum::PrefixSum, prelude::*};

pub fn part1(input: &PuzzleInput) -> String {
    solve(input, 2)
}

pub fn solve(input: &PuzzleInput, expansion: usize) -> String {
    let empty_rows = empty_rows(&input.grid);
    let empty_columns = empty_columns(&input.grid);

    let prefix_sum_rows = make_prefix_sum(input.grid.height(), empty_rows, expansion);
    let prefix_sum_columns = make_prefix_sum(input.grid.width(), empty_columns, expansion);

    let galaxy_coords = galaxy_coords(&input.grid);

    let mut total = 0;

    for r_comb in galaxy_coords.iter().combinations(2) {
        let (x0, x1) = (
            r_comb[0].x().min(r_comb[1].x()),
            (r_comb[0].x().max(r_comb[1].x())),
        );

        let (y0, y1) = (
            r_comb[0].y().min(r_comb[1].y()),
            (r_comb[0].y().max(r_comb[1].y())),
        );

        let x0 = x0 as usize;
        let x1 = x1 as usize;
        let y0 = y0 as usize;
        let y1 = y1 as usize;

        let delta_x = prefix_sum_columns.query(x0..x1);
        let delta_y = prefix_sum_rows.query(y0..y1);

        total += delta_x + delta_y;
    }

    total.to_string()
}

pub fn make_prefix_sum(w: usize, empties: Vec<usize>, expansion: usize) -> PrefixSum<usize> {
    let mut row_or_col = vec![1; w];

    for i in empties {
        row_or_col[i] = expansion;
    }

    PrefixSum::new(&row_or_col)
}

pub fn galaxy_coords(grid: &Grid2D<char>) -> Vec<Coordinate> {
    grid.iter()
        .filter(|(_coord, c)| **c == '#')
        .map(|(coord, _)| coord)
        .collect()
}

pub fn empty_columns(grid: &Grid2D<char>) -> Vec<usize> {
    let mut columns = Vec::new();

    for x in 0..grid.width() {
        let mut empty = true;

        for y in 0..grid.height() {
            if grid[Coordinate::new(x as i32, y as i32)] != '.' {
                empty = false;
                break;
            }
        }

        if empty {
            columns.push(x);
        }
    }

    columns
}

pub fn empty_rows(grid: &Grid2D<char>) -> Vec<usize> {
    let mut rows = Vec::new();

    for y in 0..grid.height() {
        let mut empty = true;

        for x in 0..grid.width() {
            if grid[Coordinate::new(x as i32, y as i32)] != '.' {
                empty = false;
                break;
            }
        }

        if empty {
            rows.push(y);
        }
    }

    rows
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::indoc;

    const TEST_INPUT: &str = indoc! {"
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....
    "};

    #[test]
    fn test_part1() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part1(&input), "374");
    }
}
