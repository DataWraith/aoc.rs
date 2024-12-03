use crate::structs::*;

use utility_belt::prelude::*;

#[tracing::instrument(skip(input))]
pub fn part2(input: &PuzzleInput) -> String {
    let mut count = 0;

    let mut dbg_grid = input.grid.map(|c| '.');

    for col in 0..input.grid.width {
        for row in 0..input.grid.height {
            let center = *input
                .grid
                .get(Coordinate::new(col as i32, row as i32))
                .unwrap_or(&'.');
            let tl = *input
                .grid
                .get(Coordinate::new(col as i32 - 1, row as i32 - 1))
                .unwrap_or(&'.');
            let tr = *input
                .grid
                .get(Coordinate::new(col as i32 + 1, row as i32 - 1))
                .unwrap_or(&'.');
            let bl = *input
                .grid
                .get(Coordinate::new(col as i32 - 1, row as i32 + 1))
                .unwrap_or(&'.');
            let br = *input
                .grid
                .get(Coordinate::new(col as i32 + 1, row as i32 + 1))
                .unwrap_or(&'.');

            if center == 'A' {
                let mut x = vec![tl, tr, bl, br];

                if x == vec!['M', 'S', 'M', 'S'] || x == vec!['S', 'M', 'S', 'M'] || x == vec!['M', 'M', 'S', 'S']  || x == vec!['S', 'S', 'M', 'M'] {

                //if x == vec!['M', 'M', 'S', 'S'] {
                    count += 1;

                    dbg_grid.set(Coordinate::new(col as i32, row as i32), 'A');
                    dbg_grid.set(Coordinate::new(col as i32 - 1, row as i32 - 1), tl);
                    dbg_grid.set(Coordinate::new(col as i32 + 1, row as i32 - 1), tr);
                    dbg_grid.set(Coordinate::new(col as i32 - 1, row as i32 + 1), bl);
                    dbg_grid.set(Coordinate::new(col as i32 + 1, row as i32 + 1), br);
                }
            }
        }
    }

    println!("{}", &dbg_grid);
    println!("{}", count);
    println!("{}", dbg_grid.iter().filter(|(_, c)| **c == 'A').count());

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
    fn test_part2_example() {
        let input = crate::parser::part2(TEST_INPUT);
        assert_eq!(part2(&input), "9");
    }
}
