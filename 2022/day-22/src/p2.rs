use utility_belt::prelude::*;

use crate::{p1::State, parser::*};

pub fn part2(input: &PuzzleInput) -> String {
    let regions = find_connected_regions(input, 50);

    let face_connections = [
        // Face 0
        (0, 2, Direction::Down, Direction::Down),
        (0, 1, Direction::Right, Direction::Right),
        (0, 3, Direction::Left, Direction::Right),
        (0, 5, Direction::Up, Direction::Right),
        // Face 1
        (1, 2, Direction::Down, Direction::Left),
        (1, 0, Direction::Left, Direction::Left),
        (1, 5, Direction::Up, Direction::Up),
        (1, 4, Direction::Right, Direction::Left),
        // Face 2
        (2, 4, Direction::Down, Direction::Down),
        (2, 1, Direction::Right, Direction::Up),
        (2, 3, Direction::Left, Direction::Down),
        (2, 0, Direction::Up, Direction::Up),
        // Face 3
        (3, 5, Direction::Down, Direction::Down),
        (3, 0, Direction::Left, Direction::Right),
        (3, 4, Direction::Right, Direction::Right),
        (3, 2, Direction::Up, Direction::Right),
        // Face 4
        (4, 5, Direction::Down, Direction::Left),
        (4, 3, Direction::Left, Direction::Left),
        (4, 1, Direction::Right, Direction::Left),
        (4, 2, Direction::Up, Direction::Up),
        // Face 5
        (5, 1, Direction::Down, Direction::Down),
        (5, 4, Direction::Right, Direction::Up),
        (5, 0, Direction::Left, Direction::Down),
        (5, 3, Direction::Up, Direction::Up),
    ];

    let connections = make_connections(input, 50, regions, face_connections);

    let mut state = State::new(input);

    let mut path_grid = input.costs.map(|c| {
        if c == &0 {
            ' '
        } else if c == &255 {
            '#'
        } else {
            '.'
        }
    });

    for instruction in input.instructions.iter() {
        state = state.step_connections(&connections, instruction, &mut path_grid);
    }

    let final_column = state.position.x + 1;
    let final_row = state.position.y + 1;
    let final_facing = match state.direction {
        Direction::Right => 0,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Up => 3,
        _ => unreachable!(),
    };

    dbg!(&final_row, &final_column, &final_facing);

    let password = final_row * 1000 + final_column * 4 + final_facing;

    password.to_string()
}

pub fn part2_example(input: &PuzzleInput) -> String {
    let regions = find_connected_regions(input, 4);

    let face_connections = [
        // Face 0
        (0, 3, Direction::Down, Direction::Down),
        (0, 5, Direction::Right, Direction::Left),
        (0, 2, Direction::Left, Direction::Down),
        (0, 1, Direction::Up, Direction::Down),
        // Face 1
        (1, 4, Direction::Down, Direction::Up),
        (1, 5, Direction::Left, Direction::Up),
        (1, 0, Direction::Up, Direction::Up),
        (1, 2, Direction::Right, Direction::Right),
        // Face 2
        (2, 4, Direction::Down, Direction::Right),
        (2, 3, Direction::Right, Direction::Right),
        (2, 1, Direction::Left, Direction::Left),
        (2, 0, Direction::Up, Direction::Right),
        // Face 3
        (3, 4, Direction::Down, Direction::Down),
        (3, 2, Direction::Left, Direction::Left),
        (3, 5, Direction::Right, Direction::Down),
        (3, 0, Direction::Up, Direction::Up),
        // Face 4
        (4, 1, Direction::Down, Direction::Up),
        (4, 2, Direction::Left, Direction::Up),
        (4, 5, Direction::Right, Direction::Right),
        (4, 3, Direction::Up, Direction::Up),
        // Face 5
        (5, 1, Direction::Down, Direction::Right),
        (5, 0, Direction::Right, Direction::Left),
        (5, 4, Direction::Left, Direction::Left),
        (5, 3, Direction::Up, Direction::Left),
    ];

    let connections = make_connections(input, 4, regions, face_connections);

    let mut state = State::new(input);
    let mut path_grid = input.costs.map(|c| {
        if c == &0 {
            ' '
        } else if c == &255 {
            '#'
        } else {
            '.'
        }
    });

    for instruction in input.instructions.iter() {
        state = state.step_connections(&connections, instruction, &mut path_grid);
    }

    println!("{}", path_grid);

    let final_column = state.position.x + 1;
    let final_row = state.position.y + 1;
    let final_facing = match state.direction {
        Direction::Right => 0,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Up => 3,
        _ => unreachable!(),
    };

    dbg!(&final_row, &final_column, &final_facing);

    let password = final_row * 1000 + final_column * 4 + final_facing;

    password.to_string()
}

pub fn find_connected_regions(input: &PuzzleInput, size: usize) -> [Coordinate; 6] {
    let mut regions = [Coordinate::new(0, 0); 6];
    let mut region_index = 0;

    for y in (0..input.costs.height()).step_by(size) {
        for x in (0..input.costs.width()).step_by(size) {
            let coordinate = Coordinate::new(x as i32, y as i32);

            if input.costs.get(coordinate).unwrap_or(&0) != &0 {
                regions[region_index] = coordinate;
                region_index += 1;
            }
        }
    }

    regions
}

fn make_connections(
    input: &PuzzleInput,
    size: i32,
    regions: [Coordinate; 6],
    face_connections: [(usize, usize, Direction, Direction); 6 * 4],
) -> HashMap<State, State> {
    let mut connections = HashMap::new();

    for (region_index, region) in regions.iter().enumerate() {
        for x in (region.x)..(region.x + size) {
            for y in (region.y)..(region.y + size) {
                let position = Coordinate::new(x as i32, y as i32);

                for dir in Direction::cardinal() {
                    let start = State {
                        position,
                        direction: dir,
                    };

                    let neighbor = position.neighbor(dir);

                    if input.costs.get(neighbor).unwrap_or(&0) == &255 {
                        connections.insert(start.clone(), start);
                        continue;
                    }

                    let neighbor = State {
                        position: neighbor,
                        direction: dir,
                    };

                    if input.costs.get(neighbor.position).unwrap_or(&0) != &1 {
                        continue;
                    }

                    connections.insert(start, neighbor);
                }
            }
        }

        for (c1, c2, dir1, dir2) in face_connections {
            if c1 != region_index {
                continue;
            }

            let (xa_1, xa_2, ya_1, ya_2) = border(size, region, dir1);
            let (xb_1, xb_2, yb_1, yb_2) = border(size, &regions[c2], dir2.opposite());

            let mut coordinates1 = Vec::new();

            for x in xa_1..=xa_2 {
                for y in ya_1..=ya_2 {
                    coordinates1.push(Coordinate::new(x as i32, y as i32));
                }
            }

            let mut coordinates2 = Vec::new();

            for x in xb_1..=xb_2 {
                for y in yb_1..=yb_2 {
                    coordinates2.push(Coordinate::new(x as i32, y as i32));
                }
            }

            match (dir1, dir2) {
                (Direction::Right, Direction::Left)
                | (Direction::Left, Direction::Right)
                | (Direction::Up, Direction::Down)
                | (Direction::Down, Direction::Up) => {
                    coordinates2.reverse();
                }

                (Direction::Right, Direction::Down)
                | (Direction::Down, Direction::Right)
                | (Direction::Up, Direction::Left)
                | (Direction::Left, Direction::Up) => {
                    coordinates1.reverse();
                }
                _ => {}
            }

            for (c1, c2) in coordinates1.clone().iter().zip(coordinates2.clone().iter()) {
                let mut c1 = c1.clone();
                let mut c2 = c2.clone();

                if c1 == Coordinate::new(4, 4) {
                    dbg!(&c1, &c2, dir1, dir2);
                    dbg!(&coordinates1);
                    dbg!(&coordinates2);
                }

                let start = State {
                    position: c1,
                    direction: dir1,
                };

                let end = State {
                    position: c2,
                    direction: dir2,
                };

                if c1 == c2 {
                    continue;
                }

                if start.position == Coordinate::new(11, 5) {
                    dbg!(&start);
                    dbg!(&end);
                    dbg!(&coordinates1);
                    dbg!(&coordinates2);
                }

                if input.costs.get(end.position).unwrap_or(&0) == &255 {
                    connections.insert(start.clone(), start);
                    continue;
                }

                connections.insert(start, end);
            }
        }
    }

    connections
}

fn border(size: i32, region: &Coordinate, dir1: Direction) -> (i32, i32, i32, i32) {
    match dir1 {
        Direction::Right => (
            region.x + size - 1,
            region.x + size - 1,
            region.y,
            region.y + size - 1,
        ),
        Direction::Left => (region.x, region.x, region.y, region.y + size - 1),
        Direction::Up => (region.x, region.x + size - 1, region.y, region.y),
        Direction::Down => (
            region.x,
            region.x + size - 1,
            region.y + size - 1,
            region.y + size - 1,
        ),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = indoc! {"
                ...#
                .#..
                #...
                ....
        ...#.......#
        ........#...
        ..#....#....
        ..........#.
                ...#....
                .....#..
                .#......
                ......#.

        10R5L5R10L4R5L5
    "};

    #[test]
    fn test_part2_example() {
        let input = crate::parser::part2(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(part2_example(&input), "5031");
    }
}
