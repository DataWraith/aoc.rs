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
            for dir in Direction::cardinal() {
                let pos = Coordinate::new(x as i32, y as i32);

                if grid[pos] == u32::MAX {
                    continue;
                }

                let neighbor = pos.neighbor(dir);
                let neighbor_neighbor = neighbor.neighbor(dir);

                if grid[neighbor] != u32::MAX {
                    continue;
                }

                if grid.get(neighbor_neighbor).is_none() {
                    continue;
                }

                if grid[neighbor_neighbor] == u32::MAX {
                    continue;
                }

                let cur_val = grid[pos];
                let next_val = grid[neighbor_neighbor];

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
    let end = grid.iter().find(|(_, &c)| c == 'E').unwrap().0;

    let mut queue = VecDeque::new();
    queue.push_back((end, 0));

    while let Some((current, distance)) = queue.pop_front() {
        result[current] = distance;

        if grid[current] == 'S' {
            return result;
        }

        for neighbor in current.neighbors() {
            if grid[neighbor] != '#' && result[neighbor] > distance + 1 {
                queue.push_back((neighbor, distance + 1));
            }
        }
    }

    unreachable!()
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
