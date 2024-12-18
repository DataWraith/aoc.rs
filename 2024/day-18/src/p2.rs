use std::collections::BTreeSet;

use utility_belt::prelude::*;

use crate::parser::*;

pub fn part2(input: &PuzzleInput) -> String {
    let mut memory = Grid2D::new(input.width, input.height, 0);

    let mut q = VecDeque::new();

    q.push_back((
        Coordinate::new(input.width as i32 - 1, input.height as i32 - 1),
        0,
    ));

    while let Some(c) = q.pop_front() {
        for d in Direction::cardinal() {
            let nc = c.0 + d;

            if nc == Coordinate::new(input.width as i32 - 1, input.height as i32 - 1) {
                continue;
            }

            if memory.get(nc) == Some(&0) {
                memory.set(nc, c.1 + 1);
                q.push_back((nc, c.1 + 1));
            }
        }
    }

    let max_val = 150;

    'outer: for byte_coord in input.bytes.iter() {
        if byte_coord.x == input.width as i32 - 1 && byte_coord.y == input.height as i32 - 1 {
            return format!("{},{}", byte_coord.x, byte_coord.y);
        }

        let previous_cost = memory.get(*byte_coord).unwrap();
        memory.set(*byte_coord, max_val);

        //dbg!(&memory.map(|x| if *x == 150 {
        //    "###".to_string()
        //} else {
        //    format!("{:03}", x)
        //}));
        //dbg!(&byte_coord);

        let mut q2 = VecDeque::new();
        let mut wanted_count = 0;

        for d in Direction::cardinal() {
            let nc = *byte_coord + d;

            if memory.get(nc).is_some() {
                if *memory.get(nc).unwrap() == max_val {
                    continue;
                }

                q2.push_back((nc, *memory.get(nc).unwrap(), nc));
                wanted_count += 1;
            }
        }

        let mut visisted = memory.map(|_| 500);

        let mut found_start = BTreeSet::new();
        let mut found_end = BTreeSet::new();

        while let Some((c, cost, origin)) = q2.pop_front() {
            if c == Coordinate::new(input.width as i32 - 1, input.height as i32 - 1) {
                found_end.insert(origin);
            }

            if c == Coordinate::new(0, 0) {
                found_start.insert(origin);
            }

            if *memory.get(c).unwrap() == max_val {
                continue;
            }

            if *visisted.get(c).unwrap() == cost {
                continue;
            }

            visisted.set(c, cost);

            let mut min_cost = max_val;

            for d in Direction::cardinal() {
                let nc = c + d;

                if memory.get(nc).is_some() {
                    let ncost = memory.get(nc).unwrap();
                    min_cost = min_cost.min(ncost + 1);
                }
            }

            if min_cost < max_val && min_cost >= cost {
                for d in Direction::cardinal() {
                    let nc = c + d;

                    if memory.get(nc).is_some() {
                        if *memory.get(nc).unwrap() != max_val {
                            q2.push_back((nc, *memory.get(nc).unwrap(), origin));
                        }
                    }
                }
            }

            memory.set(c, min_cost);
        }

        //dbg!("Finding path");

        q.push_back((
            Coordinate::new(0, 0),
            *memory.get(Coordinate::new(0, 0)).unwrap(),
        ));

        while let Some((c, cost)) = q.pop_front() {
            for d in Direction::cardinal() {
                let nc = c + d;

                if nc == Coordinate::new(input.width as i32 - 1, input.height as i32 - 1) {
                    q.clear();
                    continue 'outer;
                }

                if memory.get(nc) == Some(&(cost - 1)) {
                    q.push_back((nc, cost - 1));
                }
            }
        }

        return format!("{},{}", byte_coord.x, byte_coord.y);
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
