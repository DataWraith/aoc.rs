use std::cmp::Reverse;

use utility_belt::prelude::*;

use crate::parser::*;

pub fn part2(input: &PuzzleInput) -> String {
    let low_points = input
        .map
        .iter()
        .filter_map(|(coord, height)| {
            let is_low_point = coord
                .neighbors()
                .filter(|neighbor| input.map.contains(*neighbor))
                .all(|neighbor| input.map[neighbor] > *height);

            if is_low_point {
                Some(coord)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let mut basins = Vec::new();

    for low_point in low_points {
        let mut size = 0;

        let mut q = VecDeque::new();
        let mut visited = HashSet::new();
        q.push_back(low_point);

        while let Some(coord) = q.pop_front() {
            if !visited.insert(coord) {
                continue;
            }

            size += 1;

            for neighbor in coord.neighbors() {
                if input.map.contains(neighbor) && input.map[neighbor] < 9 {
                    q.push_back(neighbor);
                }
            }
        }

        basins.push(size);
    }

    let (basins, _, _) = basins.select_nth_unstable_by_key(3, |&size| Reverse(size));

    basins.iter().product::<usize>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

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
