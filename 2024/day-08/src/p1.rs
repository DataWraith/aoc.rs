use crate::structs::*;

use utility_belt::prelude::*;

#[tracing::instrument(skip(input))]
pub fn part1(input: &PuzzleInput) -> String {
    let freqs = frequencies(&input.grid);
    let mut total = HashSet::new();

    for frequency in freqs {
        let coordinates = antenna_coordinates(&input.grid, frequency);
        let antinodes = all_antinodes(
            input.grid.width() as i32,
            input.grid.height() as i32,
            &coordinates,
        );

        total.extend(antinodes);
    }

    total.len().to_string()
}

pub fn frequencies(grid: &Grid2D<char>) -> HashSet<char> {
    grid.iter()
        .filter(|(_, c)| **c != '.' && **c != '#')
        .map(|(_, c)| *c)
        .collect()
}

pub fn antenna_coordinates(grid: &Grid2D<char>, antenna: char) -> HashSet<Coordinate> {
    grid.iter()
        .filter(|(_, c)| **c == antenna)
        .map(|(coord, _)| coord)
        .collect()
}

pub fn antinodes(a: &Coordinate, b: &Coordinate) -> [Coordinate; 2] {
    let dx = a.x - b.x;
    let dy = a.y - b.y;

    [
        Coordinate::new(a.x + dx, a.y + dy),
        Coordinate::new(b.x - dx, b.y - dy),
    ]
}

pub fn all_antinodes(w: i32, h: i32, coordinates: &HashSet<Coordinate>) -> HashSet<Coordinate> {
    coordinates
        .iter()
        .cloned()
        .combinations(2)
        .flat_map(|x| antinodes(&x[0], &x[1]))
        .filter(|c| c.x >= 0 && c.x < w && c.y >= 0 && c.y < h)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;


    const TEST_INPUT: &str = indoc! {"
        ............
        ........0...
        .....0......
        .......0....
        ....0.......
        ......A.....
        ............
        ............
        ........A...
        .........A..
        ............
        ............
    "};

    #[test]
    fn test_part1_example() {
        let input = crate::parser::part1(TEST_INPUT);
        assert_eq!(part1(&input), "14");
    }
}
