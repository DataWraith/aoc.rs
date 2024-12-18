use std::{collections::BTreeSet, mem};

use utility_belt::prelude::*;

use crate::parser::*;

pub fn shortest_path_exists(grid: &Grid2D<i32>, start: Coordinate) -> Option<Grid2D<i32>> {
    let mut memory = grid.clone();

    let mut q = VecDeque::new();

    q.push_back((
        Coordinate::new(grid.width as i32 - 1, grid.height as i32 - 1),
        1,
    ));

    memory.set(
        Coordinate::new(grid.width as i32 - 1, grid.height as i32 - 1),
        1,
    );

    while let Some((c, cost)) = q.pop_front() {
        if c == start {
            return Some(memory);
        }

        for d in Direction::cardinal() {
            let nc = c + d;

            if memory.get(nc) < Some(&(cost + 1)) {
                memory.set(nc, cost + 1);
                q.push_back((nc, cost + 1));
            }
        }
    }

    None
}

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

        memory.set(*byte_coord, max_val);

        for d in Direction::cardinal() {
            let nc = *byte_coord + d;

            if memory.get(nc).is_none() {
                continue;
            }

            if *memory.get(nc).unwrap() == max_val {
                continue;
            }

            for nc2 in nc.neighbors() {
                if memory.get(nc2).is_none() {
                    continue;
                }

                let cost = *memory.get(nc).unwrap();
                let neighbor_cost = *memory.get(nc2).unwrap();

                if neighbor_cost < max_val {
                    memory.set(nc, (neighbor_cost + 1).min(cost));
                }
            }
        }

        dbg!(&memory.map(|x| if *x == 999 {
            "###".to_string()
        } else {
            format!("{:03}", x)
        }));
        dbg!(&byte_coord);

        let mut q = VecDeque::new();
        q.push_back(Coordinate::new(0, 0));

        while let Some(cur) = q.pop_front() {
            if cur == Coordinate::new(input.width as i32 - 1, input.height as i32 - 1) {
                continue 'outer;
            }

            let cost = *memory.get(cur).unwrap();

            for d in Direction::cardinal() {
                let nc = cur + d;

                if memory.get(nc).is_none() {
                    continue;
                }

                let neighbor_cost = *memory.get(nc).unwrap();

                dbg!(&neighbor_cost);
                dbg!(&cost);

                if neighbor_cost + 1 == cost {
                    q.push_back(nc);
                }
            }
        }

        //let mut visisted = memory.map(|x| x + 1);
        //visisted.set(
        //Coordinate::new(input.width as i32 - 1, input.height as i32 - 1),
        //1,
        //);

        //while let Some((c, cost, origin)) = q2.pop_front() {
        ////dbg!(&memory.map(|x| if *x == 150 {
        ////"###".to_string()
        ////} else {
        ////format!("{:03}", x)
        ////}));
        ////dbg!(&byte_coord);

        //if visisted.get(c) == Some(&cost) {
        //continue;
        //}

        //visisted.set(c, cost);

        //let mut min_neighbor_cost = max_val;

        //for d in Direction::cardinal() {
        //let nc = c + d;

        //if memory.get(nc).is_none() {
        //continue;
        //}

        //let neighbor_cost = memory.get(nc).unwrap();
        //min_neighbor_cost = min_neighbor_cost.min(*neighbor_cost);
        //}

        //if min_neighbor_cost >= max_val {
        //memory.set(c, max_val);
        //continue;
        //}

        //memory.set(c, min_neighbor_cost + 1);

        //for d in Direction::cardinal() {
        //let nc = c + d;

        //if nc == origin {
        //continue;
        //}

        //if memory.get(nc).is_none() {
        //continue;
        //}

        //let neighbor_cost = memory.get(nc).unwrap();

        //if *neighbor_cost == min_neighbor_cost {
        //q2.push_back((nc, *neighbor_cost, c));
        //}
        //}

        ////if c == Coordinate::new(input.width as i32 - 1, input.height as i32 - 1) {
        ////found_end.insert(origin);
        ////continue;
        ////}

        ////if c == Coordinate::new(0, 0) {
        ////found_start.insert(origin);
        ////}

        ////if *memory.get(c).unwrap() == max_val {
        ////continue;
        ////}

        ////if *visisted.get(c).unwrap() == cost {
        ////continue;
        ////}

        ////visisted.set(c, cost);

        ////let mut min_cost = max_val;

        ////for d in Direction::cardinal() {
        ////let nc = c + d;

        ////if memory.get(nc).is_some() {
        ////let ncost = memory.get(nc).unwrap();
        ////min_cost = min_cost.min(ncost + 1);
        ////}
        ////}

        ////let min_cost_changed = min_cost != cost;

        ////if min_cost < max_val && min_cost >= cost {
        ////for d in Direction::cardinal() {
        ////let nc = c + d;

        ////if memory.get(nc).is_some() {
        ////if *memory.get(nc).unwrap() != max_val {
        ////q2.push_back((nc, *memory.get(nc).unwrap(), origin));
        ////}
        ////}
        ////}
        ////}

        ////memory.set(c, min_cost);
        //}

        ////if found_start.union(&found_end).count() == 0 {
        ////    dbg!(&memory.map(|x| if *x == 150 {
        ////        "###".to_string()
        ////    } else {
        ////        format!("{:03}", x)
        ////    }));
        ////    dbg!(&byte_coord);
        ////}

        ////dbg!("Finding path");

        //q.clear();
        //q.push_back((
        //Coordinate::new(0, 0),
        //*memory.get(Coordinate::new(0, 0)).unwrap(),
        //));

        //while let Some((c, cost)) = q.pop_front() {
        //for d in Direction::cardinal() {
        //let nc = c + d;

        //if nc == Coordinate::new(input.width as i32 - 1, input.height as i32 - 1) {
        //q.clear();
        //continue 'outer;
        //}

        //if memory.get(nc) == Some(&(cost - 1)) {
        //q.push_back((nc, cost - 1));
        //}
        //}
        //}

        dbg!(&memory.map(|x| if *x == 999 {
            "###".to_string()
        } else {
            format!("{:03}", x)
        }));
        dbg!(&byte_coord);

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
