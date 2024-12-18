use std::collections::HashSet;

use utility_belt::prelude::*;

use crate::parser::*;

pub fn part2(input: &PuzzleInput) -> String {
    let mut memory = Grid2D::new(input.width, input.height, false);
    let mut path: Option<HashSet<Coordinate>> = None;

    // Simulate the falling bytes
    for byte_coord in input.bytes.iter() {
        // If the byte falls on our exit, we're trapped!
        if byte_coord.x == input.width as i32 - 1 && byte_coord.y == input.height as i32 - 1 {
            return format!("{},{}", byte_coord.x, byte_coord.y);
        }

        // Mark the byte as fallen
        memory.set(*byte_coord, true);

        // If we don't have a valid path (either because this is the first invocation, or because a byte fell onto our path),
        // find the shortest path to the bottom right using A*
        if path.is_none() || path.clone().unwrap().contains(byte_coord) {
            let start = Coordinate::new(0, 0);
            let end = Coordinate::new(input.width as i32 - 1, input.height as i32 - 1);
            let mut visited = memory.map(|_| false);

            // This is a bit ugly, but it appeases the borrow checker
            let m2 = memory.clone();

            // Define the successors function that returns the valid neighbors of the current coordinate
            let successors = move |c: &Coordinate| {
                if *c == end {
                    return vec![];
                }

                let neighbors = c
                    .neighbors()
                    .filter(|n| m2.get(*n).is_some())
                    .filter(|n| !(*m2.get(*n).unwrap()))
                    .filter(|n| !(*visited.get(*n).unwrap()))
                    .map(|n| (n, 1))
                    .collect_vec();

                // Mark the neighbors as visited
                neighbors.iter().for_each(|n| {
                    visited.set(n.0, true);
                });

                neighbors
            };

            // And then just A* and convert the path into a set of coordinates
            path = astar(
                &start,
                successors,
                |c| *c == end,
                |c| c.manhattan_distance(end),
            )
            .map(|(path, _cost)| path.iter().cloned().collect::<HashSet<_>>());
        }

        // If we don't have a valid path, the last byte that fell is our puzzle answer.
        if path.is_none() {
            return format!("{},{}", byte_coord.x, byte_coord.y);
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

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
