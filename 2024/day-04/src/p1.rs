use crate::structs::*;

#[tracing::instrument(skip(input))]
pub fn part1(input: &PuzzleInput) -> String {
    let mut count = 0;

    let xmas = ['X', 'M', 'A', 'S'];

    for col in 0..input.grid.width {
        for row in 0..input.grid.height {
            for dx in [-1i32, 0, 1] {
                'outer: for dy in [-1i32, 0, 1] {
                    if dx == 0 && dy == 0 {
                        continue;
                    }

                    for (i, &x) in xmas.iter().enumerate() {
                        let c = input
                            .grid
                            .get((col + dx * i as i32, row + dy * i as i32).into())
                            .unwrap_or(&'.');

                        if c != &x {
                            continue 'outer;
                        }
                    }

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
    fn test_part1_example() {
        let input = crate::parser::part1(TEST_INPUT);
        assert_eq!(part1(&input), "18");
    }
}
