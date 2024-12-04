use crate::structs::*;

#[tracing::instrument(skip(input))]
pub fn part2(input: &PuzzleInput) -> String {
    let mut count = 0;

    for col in 1..(input.grid.width - 1) {
        for row in 1..(input.grid.height - 1) {
            let center = input.grid[(col, row).into()];
            let tl = input.grid[(col - 1, row - 1).into()];
            let tr = input.grid[(col + 1, row - 1).into()];
            let bl = input.grid[(col - 1, row + 1).into()];
            let br = input.grid[(col + 1, row + 1).into()];

            if center == 'A' {
                let x = (tl, tr, bl, br);

                if x == ('M', 'S', 'M', 'S')
                    || x == ('S', 'M', 'S', 'M')
                    || x == ('M', 'M', 'S', 'S')
                    || x == ('S', 'S', 'M', 'M')
                {
                    count += 1;
                }
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
