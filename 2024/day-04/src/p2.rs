
use crate::structs::*;

#[tracing::instrument(skip(input))]
pub fn part2(input: &PuzzleInput) -> String {
    let mut count = 0;

    for col in 1..(input.grid.width - 1) {
        for row in 1..(input.grid.height - 1) {
            let center = *input.grid.get((col, row).into()).unwrap();
            let tl = *input.grid.get((col - 1, row - 1).into()).unwrap();
            let tr = *input.grid.get((col + 1, row - 1).into()).unwrap();
            let bl = *input.grid.get((col - 1, row + 1).into()).unwrap();
            let br = *input.grid.get((col + 1, row + 1).into()).unwrap();

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
