use crate::structs::*;

use utility_belt::prelude::*;

pub fn part1(input: &PuzzleInput) -> String {
    walk(input, 64, false).to_string()
}

pub fn find_start(input: &PuzzleInput) -> Coordinate {
    input
        .grid
        .iter()
        .find(|(coord, c)| **c == 'S' && coord.x() == coord.y())
        .map(|(c, _)| c)
        .unwrap()
}

pub fn floodfill(input: &PuzzleInput, max_dist: usize, wrap: bool) -> HashMap<Coordinate, usize> {
    let start = find_start(input);
    let mut dist = 0;

    let mut result = HashMap::default();
    let mut queue = VecDeque::from([start]);

    result.insert(start, 0);

    while !queue.is_empty() && dist < max_dist {
        dist += 1;

        for _ in 0..queue.len() {
            let current = queue.pop_front().unwrap();

            for neighbor in current.neighbors() {
                let x = neighbor.x().rem_euclid(input.grid.width() as i32);
                let y = neighbor.y().rem_euclid(input.grid.height() as i32);
                let c = Coordinate::new(x, y);

                if !wrap && c != neighbor {
                    continue;
                }

                if input.grid[c] == 'S' {
                    println!("Found S at {}: {}", c, dist);
                }

                if input.grid[c] == '#' || result.contains_key(&neighbor) {
                    continue;
                }

                result.insert(neighbor, dist);
                queue.push_back(neighbor);
            }
        }
    }

    result
}

pub fn walk(input: &PuzzleInput, steps: usize, wrap: bool) -> usize {
    let distances = floodfill(input, steps, wrap);

    // If the distance is e.g. even, then we neeed an even number of steps
    // to get there, and we don't count the odd numbered tiles, and vice
    // versa.
    distances.values().filter(|&d| d % 2 == steps % 2).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = indoc! {"
        ...........
        .....###.#.
        .###.##..#.
        ..#.#...#..
        ....#.#....
        .##..S####.
        .##..#...#.
        .......##..
        .##.#.####.
        .##..##.##.
        ...........
    "};

    #[test]
    fn test_part1() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(walk(&input, 1, false), 2);
        assert_eq!(walk(&input, 2, false), 4);
        assert_eq!(walk(&input, 3, false), 6);
        assert_eq!(walk(&input, 6, false), 16);
    }
}
