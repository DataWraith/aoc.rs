use crate::structs::*;

use utility_belt::prelude::*;

#[tracing::instrument(skip(input))]
pub fn part1(input: &PuzzleInput) -> String {
    let mut count = 0;

    for col in 0..input.grid.width {
        for row in 0..input.grid.height {
            for dx in [-1, 0, 1] {
                for dy in [-1, 0, 1] {
                    if dx == 0 && dy == 0 {
                        continue;
                    }

                    let a = *input
                        .grid
                        .get(Coordinate::new(col as i32, row as i32))
                        .unwrap_or(&'.');
                    let b = *input
                        .grid
                        .get(Coordinate::new(col as i32 + 1 * dx, row as i32 + 1 * dy))
                        .unwrap_or(&'.');
                    let c = *input
                        .grid
                        .get(Coordinate::new(col as i32 + 2 * dx, row as i32 + 2 * dy))
                        .unwrap_or(&'.');
                    let d = *input
                        .grid
                        .get(Coordinate::new(col as i32 + 3 * dx, row as i32 + 3 * dy))
                        .unwrap_or(&'.');

                    if a == 'X' && b == 'M' && c == 'A' && d == 'S' {
                        count += 1;
                    }
                }
            }
        }
    }

    count.to_string()
}

pub fn count_horizontal(input: &Grid2D<char>) -> usize {
    let mut count = 0;

    for row in 0..input.height {
        for col in 0..input.width {
            let a = *input
                .get(Coordinate::new(col as i32, row as i32))
                .unwrap_or(&'.');
            let b = *input
                .get(Coordinate::new(col as i32 + 1, row as i32))
                .unwrap_or(&'.');
            let c = *input
                .get(Coordinate::new(col as i32 + 2, row as i32))
                .unwrap_or(&'.');
            let d = *input
                .get(Coordinate::new(col as i32 + 3, row as i32))
                .unwrap_or(&'.');

            if a == 'X' && b == 'M' && c == 'A' && d == 'S' {
                count += 2;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = indoc! {"
        MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX
    "};

    #[test]
    fn test_part1_example() {
        let input = crate::parser::part1(TEST_INPUT);
        assert_eq!(part1(&input), "18");
    }
}
