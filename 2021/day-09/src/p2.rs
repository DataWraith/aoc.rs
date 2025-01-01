use std::cmp::Reverse;

use utility_belt::prelude::*;

use crate::{p1::get_low_points, parser::*};

pub fn part2(input: &PuzzleInput) -> String {
    let low_points = get_low_points(input);

    let mut basin_sizes = low_points
        .iter()
        .map(|low_point| basin_size(input, *low_point))
        .collect::<Vec<_>>();

    // Find the top 3 basins by size
    let (basins, _, _) = basin_sizes.select_nth_unstable_by_key(3, |&size| Reverse(size));

    basins.iter().product::<usize>().to_string()
}

// Floodfill the basin from the low point to get the size
fn basin_size(input: &PuzzleInput, low_point: Coordinate) -> usize {
    let mut size = 0;

    let mut q = VecDeque::new();
    let mut visited = BoolGrid2D::new(input.map.width(), input.map.height());
    q.push_back(low_point);

    while let Some(coord) = q.pop_front() {
        if visited[coord] {
            continue;
        }

        visited[coord] = true;
        size += 1;

        for neighbor in coord.neighbors() {
            if input.map.contains(neighbor) && input.map[neighbor] < 9 {
                q.push_back(neighbor);
            }
        }
    }

    size
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = indoc! {"
        2199943210
        3987894921
        9856789892
        8767896789
        9899965678
    "};

    #[test]
    fn test_part2_example() {
        let input = crate::parser::part2(TEST_INPUT);
        assert_ne!(TEST_INPUT.trim(), "TODO");
        assert_eq!(part2(&input), "1134");
    }
}
