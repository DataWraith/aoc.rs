use utility_belt::prelude::*;

use crate::{p1::shortest_path_grid, parser::*};

pub fn part2(input: &PuzzleInput) -> String {
    let path_grid = shortest_path_grid(&input.maze);
    let mut result = 0;

    for x in 1..(path_grid.width() - 1) {
        for y in 1..(path_grid.height() - 1) {
            let pos = Coordinate::new(x as i32, y as i32);

            if path_grid[pos] == u32::MAX {
                continue;
            }

            result += find_cheats(&path_grid, pos);
        }
    }

    result.to_string()
}

fn find_cheats(grid: &Grid2D<u32>, start_pos: Coordinate) -> usize {
    let mut result = 0;

    for x in 1.max(start_pos.x - 21)..(start_pos.x + 21).min(grid.width - 1) {
        for y in 1.max(start_pos.y - 21)..(start_pos.y + 21).min(grid.height - 1) {
            let pos = Coordinate::new(x as i32, y as i32);

            if grid[pos] == u32::MAX {
                continue;
            }

            let dist = pos.manhattan_distance(start_pos);

            if dist > 20 {
                continue;
            }

            let saved = grid[start_pos]
                .saturating_sub(grid[pos])
                .saturating_sub(dist as u32);

            if saved >= 100 {
                result += 1;
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
