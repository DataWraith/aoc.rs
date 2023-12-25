use crate::structs::*;

use utility_belt::prelude::{Grid2D, HashSet};

pub fn part1(input: &PuzzleInput) -> String {
    let (symbols, numbers) = split_grid(&input.grid);

    let mut sum = 0;

    symbols.iter().for_each(|(coord, c)| {
        if *c != '.' {
            let mut found = HashSet::new();

            for n in coord.moore_neighbors() {
                if let Some((number, id)) = numbers.get(n) {
                    if *id != usize::MAX && !found.contains(id) {
                        found.insert(*id);
                        sum += number;
                    }
                }
            }
        }
    });

    sum.to_string()
}

pub fn split_grid(grid: &Grid2D<char>) -> (Grid2D<char>, Grid2D<(usize, usize)>) {
    let symbol_grid = grid.map(|c| if c.is_ascii_digit() { '.' } else { *c });
    let mut numbers_grid = grid.map(|_| (0, usize::MAX));

    let mut cur = "".to_string();
    let mut coords = Vec::new();
    let mut id = 0;

    grid.iter().for_each(|(coord, c)| {
        if c.is_ascii_digit() {
            cur.push(*c);
            coords.push(coord);
        }

        // End of a number or end of the line
        if (!c.is_ascii_digit() || coord.x() as usize == symbol_grid.width() - 1) && !cur.is_empty()
        {
            let n = cur.parse::<usize>().unwrap();

            for coord in coords.iter() {
                numbers_grid[*coord] = (n, id);
            }

            cur.clear();
            coords.clear();
            id += 1;
        }
    });

    (symbol_grid, numbers_grid)
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
