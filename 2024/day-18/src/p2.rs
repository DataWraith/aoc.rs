use std::{collections::BTreeSet, mem};

use utility_belt::prelude::*;

use crate::parser::*;

pub fn part2(input: &PuzzleInput) -> String {
    let mut memory = Grid2D::new(input.width, input.height, 0);

    let mut q = VecDeque::new();

    q.push_back((
        Coordinate::new(input.width as i32 - 1, input.height as i32 - 1),
        1,
    ));

    memory.set(
        Coordinate::new(input.width as i32 - 1, input.height as i32 - 1),
        1,
    );

    while let Some(c) = q.pop_front() {
        for d in Direction::cardinal() {
            let nc = c.0 + d;

            if memory.get(nc) == Some(&0) {
                memory.set(nc, c.1 + 1);
                q.push_back((nc, c.1 + 1));
            }
        }
    }

    dbg!(&memory.map(|x| if *x == 999 {
        "###".to_string()
    } else {
        format!("{:03}", x)
    }));

    let max_val = 999;

    'outer: for byte_coord in input.bytes.iter() {
        if byte_coord.x == input.width as i32 - 1 && byte_coord.y == input.height as i32 - 1 {
            return format!("{},{}", byte_coord.x, byte_coord.y);
        }

        let prev_cost = *memory.get(*byte_coord).unwrap();
        memory.set(*byte_coord, max_val);

        q.clear();

        for d in Direction::cardinal() {
            let nc = *byte_coord + d;

            if memory.get(nc).is_none() {
                continue;
            }

            if *memory.get(nc).unwrap() == max_val {
                continue;
            }

            q.push_back((nc, prev_cost + 1));
        }

        dbg!(&memory.map(|x| if *x == 999 {
            "###".to_string()
        } else {
            format!("{:03}", x)
        }));
        dbg!(&byte_coord);

        let mut found = false;
        let mut visited = memory.map(|_| max_val);

        while let Some((cur, _cost)) = q.pop_front() {
            let cost = *memory.get(cur).unwrap();

            if cur == Coordinate::new(input.width as i32 - 1, input.height as i32 - 1) {
                found = true;
                continue;
            }

            if *visited.get(cur).unwrap() == cost {
                continue;
            }

            visited.set(cur, cost);

            if cost == max_val {
                continue;
            }

            let mut min_cost = max_val;

            for n in cur.neighbors() {
                if memory.get(n).is_none() {
                    continue;
                }

                min_cost = min_cost.min(1 + *memory.get(n).unwrap());
            }

            if min_cost == max_val {
                memory.set(cur, max_val);
            } else if *memory.get(cur).unwrap() != min_cost + 1 {
                memory.set(cur, min_cost);
                q.push_back((cur, min_cost));
            }

            if min_cost == prev_cost {
                found = true;
                continue;
            }

            for n in cur.neighbors() {
                if memory.get(n).is_none() {
                    continue;
                }

                q.push_back((n, 0)); //(min_cost + 1).min(*neighbor_cost)));
            }
        }

        if !found {
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
