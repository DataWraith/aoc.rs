use rayon::prelude::*;

use utility_belt::prelude::*;

use crate::{p1::shortest_path_grid, parser::*};

pub fn part2(input: &PuzzleInput) -> String {
    let path_grid = shortest_path_grid(&input.maze);

    let result: usize = (1..(path_grid.width() as i32 - 1))
        .into_par_iter()
        .map(|x| {
            let mut result = 0;

            for y in 1..(path_grid.height() as i32 - 1) {
                let pos = Coordinate::new(x, y);

                if path_grid[pos] == u32::MAX {
                    continue;
                }

                result += find_cheats(&path_grid, pos);
            }

            result
        })
        .sum();

    result.to_string()
}

fn find_cheats(grid: &Grid2D<u32>, orig: Coordinate) -> usize {
    let mut result = 0;

    for step_size in 2..=20 {
        for step_x in 0..=step_size {
            let step_y = step_size - step_x;

            for dx in [-1, 1] {
                for dy in [-1, 1] {
                    if step_x == 0 && dx == -1 {
                        continue;
                    }

                    if step_y == 0 && dy == -1 {
                        continue;
                    }

                    let pos = Coordinate::new(orig.x + step_x * dx, orig.y + step_y * dy);

                    if grid.get(pos).unwrap_or(&u32::MAX) == &u32::MAX {
                        continue;
                    }

                    let saved = grid[orig]
                        .saturating_sub(grid[pos])
                        .saturating_sub(step_size as u32);

                    if saved >= 100 {
                        result += 1;
                    }
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::indoc;

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
    fn test_part2_example() {
        let input = crate::parser::part2(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(part2(&input), "0");
    }
}
