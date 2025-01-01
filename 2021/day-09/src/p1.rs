use utility_belt::prelude::*;

use crate::parser::*;

pub fn part1(input: &PuzzleInput) -> String {
    let low_points = get_low_points(input);

    low_points
        .iter()
        .map(|coord| input.map[*coord] as u32 + 1)
        .sum::<u32>()
        .to_string()
}

pub fn get_low_points(input: &PuzzleInput) -> Vec<Coordinate> {
    input
        .map
        .iter()
        .filter_map(|(coord, height)| {
            if coord
                .neighbors()
                .filter(|neighbor| input.map.contains(*neighbor))
                .all(|neighbor| input.map[neighbor] > *height)
            {
                Some(coord)
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

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
