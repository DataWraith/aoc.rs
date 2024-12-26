use crate::structs::*;

use utility_belt::prelude::*;

pub fn part2(input: &PuzzleInput) -> String {
    let start = input.heightmap.iter().find(|(_, &c)| c == 'E').unwrap().0;

    let map = input.heightmap.map(|c| {
        if *c == 'S' {
            'a'
        } else if *c == 'E' {
            'z'
        } else {
            *c
        }
    });

    pathfinding::directed::astar::astar(
        &start,
        |&pos| {
            let mut neighbors = Vec::new();

            for dir in Direction::cardinal() {
                let new_pos = pos + dir;

                if let Some(elevation) = map.get(new_pos) {
                    if map[pos] as u8 > *elevation as u8 + 1 {
                        continue;
                    }

                    neighbors.push((new_pos, 1));
                }
            }

            neighbors
        },
        |_| 0,
        |&pos| map[pos] == 'a',
    )
    .unwrap()
    .1
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part2() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part2(&input), "29");
    }
}
