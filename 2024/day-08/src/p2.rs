use crate::{
    p1::{antenna_coordinates, frequencies},
    structs::*,
};

use utility_belt::prelude::*;

#[tracing::instrument(skip(input))]
pub fn part2(input: &PuzzleInput) -> String {
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

pub fn all_antinodes(w: i32, h: i32, coordinates: &HashSet<Coordinate>) -> HashSet<Coordinate> {
    coordinates
        .iter()
        .cloned()
        .combinations(2)
        .flat_map(|x| antinodes(&x[0], &x[1], w, h))
        .collect()
}

pub fn antinodes(a: &Coordinate, b: &Coordinate, width: i32, height: i32) -> Vec<Coordinate> {
    let dx = a.x - b.x;
    let dy = a.y - b.y;

    let mut result = Vec::new();

    for i in 0.. {
        let c = Coordinate::new(a.x + dx * i, a.y + dy * i);

        if c.x < 0 || c.x >= width || c.y < 0 || c.y >= height {
            break;
        }

        result.push(c);
    }

    for i in 0.. {
        let c = Coordinate::new(b.x - dx * i, b.y - dy * i);

        if c.x < 0 || c.x >= width || c.y < 0 || c.y >= height {
            break;
        }

        result.push(c);
    }

    result
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
    fn test_part2_example() {
        let input = crate::parser::part2(TEST_INPUT);
        assert_eq!(part2(&input), "34");
    }
}
