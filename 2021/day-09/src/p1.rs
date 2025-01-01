use utility_belt::prelude::*;

use crate::parser::*;

pub fn part1(input: &PuzzleInput) -> String {
    input
        .map
        .iter()
        .map(|(coord, height)| {
            let is_low_point = coord
                .neighbors()
                .filter(|neighbor| input.map.contains(*neighbor))
                .all(|neighbor| input.map[neighbor] > *height);

            if is_low_point {
                (height + 1) as u32
            } else {
                0
            }
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = indoc! {"
        2199943210
        3987894921
        9856789892
        8767896789
        9899965678
    "};

    #[test]
    fn test_part1_example() {
        let input = crate::parser::part1(TEST_INPUT);
        assert_ne!(TEST_INPUT.trim(), "TODO");
        assert_eq!(part1(&input), "15");
    }
}
