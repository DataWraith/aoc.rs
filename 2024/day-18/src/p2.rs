use std::{collections::BTreeSet, mem};

use utility_belt::prelude::*;

use crate::parser::*;

const MAX_VAL: i32 = 999;

pub fn part2(input: &PuzzleInput) -> String {
    let mut memory = Grid2D::new(input.width, input.height, false);
    let mut path: Option<HashSet<Coordinate>> = None;

    'outer: for byte_coord in input.bytes.iter() {
        if byte_coord.x == input.width as i32 - 1 && byte_coord.y == input.height as i32 - 1 {
            return format!("{},{}", byte_coord.x, byte_coord.y);
        }

        memory.set(*byte_coord, true);

        if path.is_none() || path.clone().unwrap().contains(byte_coord) {
            let start = Coordinate::new(0, 0);
            let end = Coordinate::new(input.width as i32 - 1, input.height as i32 - 1);
            let mut visited = memory.map(|_| false);
            let m2 = memory.clone();

            let successors = move |c: &Coordinate| {
                if *c == end {
                    return vec![];
                }

                let neighbors = c
                    .neighbors()
                    .filter(|n| m2.get(*n).is_some())
                    .filter(|n| *m2.get(*n).unwrap() == false)
                    .filter(|n| *visited.get(*n).unwrap() == false)
                    .map(|n| (n, 1))
                    .collect_vec();

                neighbors.iter().for_each(|n| {
                    visited.set(n.0, true);
                });

                neighbors
            };

            path = astar(
                &start,
                successors,
                |c| *c == end,
                |c| c.manhattan_distance(end),
            )
            .map(|(path, cost)| path.iter().cloned().collect::<HashSet<_>>());
        }

        if path.is_none() {
            return format!("{},{}", byte_coord.x, byte_coord.y);
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = indoc! {"
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
"};

    #[test]
    fn test_part2_example() {
        let mut input = crate::parser::part2(TEST_INPUT);
        input.width = 7;
        input.height = 7;
        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(part2(&input), "6,1");
    }
}
