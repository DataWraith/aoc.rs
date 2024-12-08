use crate::{
    p1::{antenna_coordinates, frequencies},
    structs::*,
};

use utility_belt::prelude::*;

#[tracing::instrument(skip(input))]
pub fn part2(input: &PuzzleInput) -> String {
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

pub fn antipodes(w: i32, h: i32, coordinates: &HashSet<Coordinate>) -> HashSet<Coordinate> {
    coordinates
        .iter()
        .cloned()
        .combinations(2)
        .flat_map(|x| antipode(&x[0], &x[1], w, h))
        .collect()
}

pub fn antipode(a: &Coordinate, b: &Coordinate, width: i32, height: i32) -> Vec<Coordinate> {
    let dx = a.x - b.x;
    let dy = a.y - b.y;

    let mut result = Vec::new();

    result.push(a.clone());
    result.push(b.clone());

    for i in 1.. {
        let c = Coordinate::new(a.x + dx * i as i32, a.y + dy * i as i32);
        if c.x >= 0 && c.x < width && c.y >= 0 && c.y < height {
            result.push(c);
        } else {
            break;
        }
    }

    for i in 1.. {
        let c = Coordinate::new(b.x - dx * i as i32, b.y - dy * i as i32);
        if c.x >= 0 && c.x < width && c.y >= 0 && c.y < height {
            result.push(c);
        } else {
            break;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = indoc! {"
T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
..........
"};

    #[test]
    fn test_part2_example() {
        let input = crate::parser::part2(TEST_INPUT);
        assert_eq!(part2(&input), "9");
    }
}
