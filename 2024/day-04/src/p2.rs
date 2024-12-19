use crate::structs::*;

pub fn part2(input: &PuzzleInput) -> String {
    let mut count = 0;

    for col in 1..(input.grid.width - 1) {
        for row in 1..(input.grid.height - 1) {
            let center = input.grid[(col, row).into()];

            if center != 'A' {
                continue;
            }

            let tl = input.grid[(col - 1, row - 1).into()];
            let tr = input.grid[(col + 1, row - 1).into()];
            let bl = input.grid[(col - 1, row + 1).into()];
            let br = input.grid[(col + 1, row + 1).into()];

            let diag1 = (tl, center, br);
            let diag2 = (tr, center, bl);

            if (diag1 == ('M', 'A', 'S') || diag1 == ('S', 'A', 'M'))
                && (diag2 == ('M', 'A', 'S') || diag2 == ('S', 'A', 'M'))
            {
                count += 1;
            }
        }
    }

    count.to_string()
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
    fn test_part2_example() {
        let input = crate::parser::part2(TEST_INPUT);
        assert_eq!(part2(&input), "9");
    }
}
