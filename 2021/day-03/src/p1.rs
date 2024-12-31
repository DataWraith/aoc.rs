use utility_belt::prelude::*;

use crate::parser::*;

pub fn part1(input: &PuzzleInput) -> String {
    let gamma = input
        .report
        .col_iter()
        .map(|col| {
            if col.iter().filter(|&&b| b).count() > col.len() / 2 {
                '1'
            } else {
                '0'
            }
        })
        .collect::<String>();

    let epsilon = input
        .report
        .col_iter()
        .map(|col| {
            if col.iter().filter(|&&b| b).count() > col.len() / 2 {
                '0'
            } else {
                '1'
            }
        })
        .collect::<String>();

    let gamma = u32::from_str_radix(&gamma, 2).unwrap();
    let epsilon = u32::from_str_radix(&epsilon, 2).unwrap();

    (gamma * epsilon).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = indoc! {"
        00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010
    "};

    #[test]
    fn test_part1_example() {
        let input = crate::parser::part1(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(part1(&input), "198");
    }
}
