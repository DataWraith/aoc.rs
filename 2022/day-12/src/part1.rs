use crate::structs::*;

use utility_belt::prelude::*;

pub fn part1(input: &PuzzleInput) -> String {
    let start = input.heightmap.iter().find(|(_, &c)| c == 'S').unwrap().0;
    let goal = input.heightmap.iter().find(|(_, &c)| c == 'E').unwrap().0;

    let map = input.heightmap.map(|c| {
        if *c == 'S' {
            'a'
        } else if *c == 'E' {
            'z'
        } else {
            *c
        }
    });

    astar(
        &start,
        |&pos| {
            let mut neighbors = Vec::new();

            for dir in Direction::all() {
                let new_pos = pos + dir.into();

                if let Some(elevation) = map.get(new_pos) {
                    if *elevation as u8 > map[pos] as u8 + 1 {
                        continue;
                    }

                    neighbors.push((new_pos, 1));
                }
            }

            neighbors
        },
        |&pos| pos == goal,
        |&pos| pos.manhattan_distance(goal),
    )
    .unwrap()
    .1
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part1(&input), "31");
    }
}
