use utility_belt::prelude::*;

use crate::parser::*;

pub fn part1(input: &PuzzleInput) -> String {
    let path_grid = shortest_path_grid(&input.maze);
    let result = find_cheats(&path_grid);

    result.to_string()
}

pub fn find_cheats(grid: &Grid2D<u32>) -> usize {
    let mut result = 0;

    for x in 1..(grid.width() - 1) {
        for y in 1..(grid.height() - 1) {
            let pos = Coordinate::new(x as i32, y as i32);

            if grid[pos] == u32::MAX {
                continue;
            }

            // We don't need to check diagonals because there are no diagonally
            // connected free cells in the grid.
            for dir in Direction::cardinal() {
                let neighbor = pos + dir + dir;

                if grid.get(neighbor).unwrap_or(&u32::MAX) == &u32::MAX {
                    continue;
                }

                let cur_val = grid[pos];
                let next_val = grid[neighbor];

                if cur_val.saturating_sub(next_val).saturating_sub(2) >= 100 {
                    result += 1
                }
            }
        }
    }

    result
}

pub fn shortest_path_grid(grid: &Grid2D<char>) -> Grid2D<u32> {
    let mut result = grid.map(|_| u32::MAX);
    let end = grid.iter().find(|(_, &c)| c == 'S').unwrap().0;

    let mut cur = end;
    let mut distance = 0;

    loop {
        result[cur] = distance;

        if grid[cur] == 'E' {
            return result;
        }

        for neighbor in cur.neighbors() {
            if grid[neighbor] != '#' && result[neighbor] > distance + 1 {
                cur = neighbor;
                distance += 1;
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = indoc! {"
        ###############
        #...#...#.....#
        #.#.#.#.#.###.#
        #S#...#.#.#...#
        #######.#.#.###
        #######.#.#...#
        #######.#.###.#
        ###..E#...#...#
        ###.#######.###
        #...###...#...#
        #.#####.#.###.#
        #.#...#.#.#...#
        #.#.#.#.#.#.###
        #...#...#...###
        ###############
    "};

    #[test]
    fn test_part1_example() {
        let input = crate::parser::part1(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(part1(&input), "0");
    }
}
