use crate::structs::*;

use utility_belt::prelude::*;

#[tracing::instrument(skip(input))]
pub fn part1(input: &PuzzleInput) -> String {
    let antennas = frequencies(&input.grid);
    let mut total = HashSet::new();

    for frequency in antennas {
        let coordinates = antenna_coordinates(&input.grid, frequency);
        let antipodes = antipodes(
            input.grid.width() as i32,
            input.grid.height() as i32,
            &coordinates,
        );

        total.extend(antipodes);
    }

    let mut dbg_grid = Grid2D::new(input.grid.width(), input.grid.height(), '.');

    for t in total.iter() {
        println!("{}", t);
        dbg_grid.set(*t, '#');
    }

    for (c, a) in input.grid.iter() {
        if *a != '.' && *a != '#' {
            dbg_grid.set(c.clone(), *a);
        }
    }

    println!("{}", &dbg_grid);

    total.len().to_string()
}

pub fn frequencies(grid: &Grid2D<char>) -> HashSet<char> {
    grid.iter()
        .filter(|(_, c)| **c != '.')
        .map(|(_, c)| c.clone())
        .collect()
}

pub fn antenna_coordinates(grid: &Grid2D<char>, antenna: char) -> HashSet<Coordinate> {
    grid.iter()
        .filter(|(_, c)| **c == antenna)
        .map(|(coord, _)| coord.clone())
        .collect()
}

pub fn antipode(a: &Coordinate, b: &Coordinate) -> [Coordinate; 2] {
    let dx = a.x - b.x;
    let dy = a.y - b.y;

    [
        Coordinate::new(a.x + dx as i32, a.y + dy as i32),
        Coordinate::new(b.x - dx as i32, b.y - dy as i32),
    ]
}

pub fn antipodes(w: i32, h: i32, coordinates: &HashSet<Coordinate>) -> HashSet<Coordinate> {
    coordinates
        .iter()
        .cloned()
        .combinations(2)
        .flat_map(|x| antipode(&x[0], &x[1]))
        .filter(|c| c.x >= 0 && c.x < w && c.y >= 0 && c.y < h)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

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
