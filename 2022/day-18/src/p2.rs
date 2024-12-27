use utility_belt::prelude::*;

use crate::{p1::NEIGHBOR_OFFSETS, parser::*};

pub fn part2(input: &PuzzleInput) -> String {
    // Find the dimensions of the volume containing the lava
    let mut min = IVec3::splat(i32::MAX);
    let mut max = IVec3::splat(i32::MIN);

    for cube in input.cubes.iter() {
        min = min.min(*cube);
        max = max.max(*cube);
    }

    // Add a 1-wide border around the lava to make the BFS easier
    min -= IVec3::new(1, 1, 1);
    max += IVec3::new(1, 1, 1);

    // Breadth-first search to find all the air spaces surrounding the lava
    let mut air = HashSet::new();

    let mut q = VecDeque::new();
    q.push_back(min);

    while let Some(cur) = q.pop_front() {
        for offset in NEIGHBOR_OFFSETS {
            let neighbor = cur + offset;

            if neighbor.x < min.x
                || neighbor.x > max.x
                || neighbor.y < min.y
                || neighbor.y > max.y
                || neighbor.z < min.z
                || neighbor.z > max.z
            {
                continue;
            }

            // Either the neighbor is lava or it's already been visited
            if input.cubes.contains(&neighbor) || air.contains(&neighbor) {
                continue;
            }

            air.insert(neighbor);
            q.push_back(neighbor);
        }
    }

    let mut total_sides = 0;

    for cube in input.cubes.iter() {
        let mut sides = 6;

        for offset in NEIGHBOR_OFFSETS {
            let neighbor = *cube + offset;

            if input.cubes.contains(&neighbor) || !air.contains(&neighbor) {
                sides -= 1;
            }
        }

        total_sides += sides;
    }

    total_sides.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = indoc! {"
        2,2,2
        1,2,2
        3,2,2
        2,1,2
        2,3,2
        2,2,1
        2,2,3
        2,2,4
        2,2,6
        1,2,5
        3,2,5
        2,1,5
        2,3,5
    "};

    #[test]
    fn test_part2_example() {
        let input = crate::parser::part2(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(part2(&input), "58");
    }
}
