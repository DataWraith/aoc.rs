use std::{cmp::Reverse, collections::BinaryHeap};

use crate::structs::*;

use utility_belt::prelude::*;

pub fn part1(input: &PuzzleInput) -> String {
    let start = Crucible {
        direction: None,
        position: Coordinate::new(0, 0),
        cur_straight: 0,
        min_straight: 1,
        max_straight: 3,
    };

    solve(input, start).to_string()
}

pub fn solve(input: &PuzzleInput, start: Crucible) -> usize {
    let mut queue = BinaryHeap::new();
    let mut visited = HashSet::default();

    let goal = Coordinate::new(
        input.grid.width() as i32 - 1,
        input.grid.height() as i32 - 1,
    );

    let h = |c: &Coordinate| (goal.x().abs_diff(c.x()) + goal.y().abs_diff(c.y())) as usize;

    queue.push(Reverse((h(&start.position), 0, start)));

    while let Some(Reverse((_f, cost, crucible))) = queue.pop() {
        if crucible.position == goal && (crucible.cur_straight >= crucible.min_straight) {
            return cost;
        }

        if visited.contains(&crucible) {
            continue;
        }

        visited.insert(crucible.clone());

        for (next_crucible, next_cost) in successors(input, &crucible) {
            let h = h(&next_crucible.position);
            let g = cost + next_cost;

            queue.push(Reverse((g + h, g, next_crucible)));
        }
    }

    panic!("No path found");
}

pub fn successors(input: &PuzzleInput, crucible: &Crucible) -> ArrayVec<[(Crucible, usize); 3]> {
    let mut successors = ArrayVec::new();

    if crucible.direction.is_none() {
        // I'm a little unhappy with having to special-case the first move, but
        // I can't think of a better way to do it. I tried starting to the left
        // of the grid, but that's even more inconvenient, because you'd have to
        // account for that in the "minimum/maximum straight moves" logic.
        let right = crucible.position + Direction::Right.into();

        successors.push((
            Crucible {
                direction: Some(Direction::Right),
                position: right,
                cur_straight: 1,
                ..crucible.clone()
            },
            input.grid[right].0 as usize,
        ));

        let down = crucible.position + Direction::Down.into();

        successors.push((
            Crucible {
                direction: Some(Direction::Down),
                position: down,
                cur_straight: 1,
                ..crucible.clone()
            },
            input.grid[down].0 as usize,
        ));

        return successors;
    }

    if crucible.cur_straight >= crucible.min_straight {
        // Turn Left
        let left = crucible.direction.unwrap().turn_left();
        let left_coord = crucible.position + left.into();

        match input.grid.get(left_coord) {
            None => {}
            Some(hl) => {
                successors.push((
                    Crucible {
                        direction: Some(left),
                        position: left_coord,
                        cur_straight: 1,
                        ..crucible.clone()
                    },
                    hl.0 as usize,
                ));
            }
        }

        // Turn Right
        let right = crucible.direction.unwrap().turn_right();
        let right_coord = crucible.position + right.into();

        match input.grid.get(right_coord) {
            None => {}
            Some(hl) => {
                successors.push((
                    Crucible {
                        direction: Some(right),
                        position: right_coord,
                        cur_straight: 1,
                        ..crucible.clone()
                    },
                    hl.0 as usize,
                ));
            }
        }
    }

    if crucible.cur_straight < crucible.max_straight {
        // Go Straight
        let straight_coord = crucible.position + crucible.direction.unwrap().into();

        match input.grid.get(straight_coord) {
            None => {}
            Some(hl) => {
                successors.push((
                    Crucible {
                        position: straight_coord,
                        cur_straight: crucible.cur_straight + 1,
                        ..crucible.clone()
                    },
                    hl.0 as usize,
                ));
            }
        }
    }

    successors
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = indoc! {"
        2413432311323
        3215453535623
        3255245654254
        3446585845452
        4546657867536
        1438598798454
        4457876987766
        3637877979653
        4654967986887
        4564679986453
        1224686865563
        2546548887735
        4322674655533
    "};

    #[test]
    fn test_part1() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part1(&input), "102");
    }
}
