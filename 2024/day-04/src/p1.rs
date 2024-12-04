use crate::structs::*;

#[tracing::instrument(skip(input))]
pub fn part1(input: &PuzzleInput) -> String {
    let mut count = 0;

    let mut grid = input.grid.clone();

    // Rows forwards and backwards
    for row in grid.row_iter() {
        let row = row.to_owned().to_vec();
        count += count_xmas(&row);
    }

    // Columns forwards and backwards
    for col in grid.col_iter() {
        let col = col.to_owned().to_vec();
        count += count_xmas(&col);
    }

    // Diagonals top-right to bottom-left
    for diag in grid.diagonals() {
        count += count_xmas(&diag);
    }

    grid.rotate_right();

    // Diagonals top-left to bottom-right
    for diag in grid.diagonals() {
        count += count_xmas(&diag);
    }

    count.to_string()
}

pub fn count_xmas(row: &[char]) -> usize {
    let mut count = 0;

    for w in row.windows(4) {
        if w == ['X', 'M', 'A', 'S'] || w == ['S', 'A', 'M', 'X'] {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = utility_belt::prelude::indoc! {"
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
