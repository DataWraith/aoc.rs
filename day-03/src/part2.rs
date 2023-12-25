use crate::{
    part1::{filter_adjacent_numbers, find_adjacent_numbers, parse_number_at},
    structs::*,
};

use utility_belt::prelude::*;

pub fn part2(input: &PuzzleInput) -> String {
    find_potential_gears(&input.grid)
        .iter()
        .map(|c| find_adjacent_numbers(&input.grid, *c))
        .map(|coords| filter_adjacent_numbers(&coords))
        .filter(|coords| coords.len() == 2)
        .map(|coords| {
            coords
                .into_iter()
                .map(|coord| dbg!(parse_number_at(&input.grid, coord)))
                .product::<usize>()
        })
        .sum::<usize>()
        .to_string()
}

fn find_potential_gears(grid: &Grid2D<char>) -> Vec<Coordinate> {
    let mut gears = Vec::new();

    for (coord, c) in grid.iter() {
        if *c == '*' {
            gears.push(coord);
        }
    }

    gears
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = indoc! {"
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
    "};

    #[test]
    fn test_part2() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part2(&input), "467835");
    }
}
