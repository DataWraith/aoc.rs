use utility_belt::prelude::*;

use crate::{p1::shortest_path_grid, parser::*};

pub fn part2(input: &PuzzleInput) -> String {
    let path_grid = shortest_path_grid(&input.maze);
    let mut cheat_collection: HashMap<u32, HashSet<(Coordinate, Coordinate)>> = HashMap::new();

    for x in 1..(path_grid.width() - 1) {
        for y in 1..(path_grid.height() - 1) {
            let pos = Coordinate::new(x as i32, y as i32);

            let cheats = find_cheats(&path_grid, pos);
            for (dist, pairs) in cheats.iter() {
                cheat_collection.entry(*dist).or_default().extend(pairs);
            }
        }
    }

    let mut result = 0;

    for (dist, pairs) in cheat_collection.iter() {
        if dist >= &100 {
            result += pairs.len();
        }
    }

    result.to_string()
}

fn find_cheats(
    grid: &Grid2D<u32>,
    start_pos: Coordinate,
) -> HashMap<u32, HashSet<(Coordinate, Coordinate)>> {
    if grid[start_pos] == u32::MAX {
        return HashMap::new();
    }

    let mut q = VecDeque::new();
    q.push_back((start_pos, 0u32));

    let mut result: HashMap<u32, HashSet<(Coordinate, Coordinate)>> = HashMap::new();
    let mut visited = HashSet::new();

    while let Some((pos, dist)) = q.pop_front() {
        if dist >= 20 {
            break;
        }

        if !visited.insert(pos) {
            continue;
        }

        for dir in Direction::cardinal() {
            let neighbor = pos.neighbor(dir);

            if grid.get(neighbor).is_none() {
                continue;
            }

            if grid[neighbor] != u32::MAX {
                let saved = grid[start_pos]
                    .saturating_sub(grid[neighbor])
                    .saturating_sub(start_pos.manhattan_distance(neighbor) as u32);

                if saved > 0 {
                    result
                        .entry(saved)
                        .or_default()
                        .insert((start_pos, neighbor));
                }
            }

            q.push_back((neighbor, dist + 1));
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
