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
                        .get((col as i32, row as i32).into())
                        .unwrap_or(&'.');

                    let b = *input
                        .grid
                        .get((col as i32 + 1 * dx, row as i32 + 1 * dy).into())
                        .unwrap_or(&'.');

                    let c = *input
                        .grid
                        .get((col as i32 + 2 * dx, row as i32 + 2 * dy).into())
                        .unwrap_or(&'.');

                    let d = *input
                        .grid
                        .get((col as i32 + 3 * dx, row as i32 + 3 * dy).into())
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
