use crate::structs::*;

use utility_belt::prelude::*;

pub fn part1(input: &PuzzleInput) -> String {
    find_symbols(&input.grid)
        .iter()
        .map(|c| find_adjacent_numbers(&input.grid, *c))
        .map(|coords| filter_adjacent_numbers(&coords))
        .flat_map(|coords| {
            coords
                .iter()
                .map(|c| parse_number_at(&input.grid, *c))
                .collect::<Vec<_>>()
        })
        .sum::<usize>()
        .to_string()
}

fn find_symbols(grid: &Grid2D<char>) -> Vec<Coordinate> {
    let mut symbols = Vec::new();

    for (coord, c) in grid.iter() {
        if *c != '.' && !c.is_ascii_digit() {
            symbols.push(coord);
        }
    }

    symbols
}

pub fn find_adjacent_numbers(grid: &Grid2D<char>, coord: Coordinate) -> Vec<Coordinate> {
    let mut number_coords = Vec::new();

    for n in coord.moore_neighbors() {
        if let Some(c) = grid.get(n) {
            if c.is_ascii_digit() {
                number_coords.push(n);
            }
        }
    }

    number_coords
}

pub fn filter_adjacent_numbers(coords: &[Coordinate]) -> Vec<Coordinate> {
    let mut result = Vec::new();
    let mut added = false;

    if coords.len() == 1 {
        return coords.to_vec();
    }

    let mut coords = coords.to_vec();
    coords.sort_by_key(|c| (c.y(), c.x()));

    for w in coords.windows(2) {
        let c1 = w[0];
        let c2 = w[1];

        if !added {
            added = true;
            result.push(c1);
        }

        if c1.y() == c2.y() && c1.x() + 1 == c2.x() {
            continue;
        }

        added = false;
    }

    if !added {
        result.push(*coords.last().unwrap());
    }

    result
}

pub fn parse_number_at(grid: &Grid2D<char>, start: Coordinate) -> usize {
    let mut digits = String::new();
    let mut coord = start;

    loop {
        let c = grid[coord];

        if c.is_ascii_digit() {
            if coord.x() == 0 {
                break;
            }

            coord += Direction::Left.into();
        } else {
            coord += Direction::Right.into();
            break;
        }
    }

    while coord.x() < grid.width() as i32 {
        let c = grid[coord];

        if c.is_ascii_digit() {
            digits.push(c);
            coord += Direction::Right.into();
        } else {
            break;
        }
    }

    digits.parse().unwrap()
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
    fn test_part1() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part1(&input), "4361");
    }
}
