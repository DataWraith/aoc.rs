use utility_belt::prelude::*;

use crate::parser::*;

#[tracing::instrument(skip(input))]
pub fn part2(input: &PuzzleInput) -> String {
    let mut warehouse = Grid2D::new(input.warehouse.width() * 2, input.warehouse.height(), '.');

    input.warehouse.iter().for_each(|(coord, c)| {
        let x = coord.x * 2;
        let y = coord.y;

        if *c == 'O' {
            warehouse.set((x, y).into(), '[');
            warehouse.set((x + 1, y).into(), ']');
        } else if *c == '@' {
            warehouse.set((x, y).into(), '@');
            warehouse.set((x + 1, y).into(), '.');
        } else {
            warehouse.set((x, y).into(), *c);
            warehouse.set((x + 1, y).into(), *c);
        }
    });

    let result = run_robot(&PuzzleInput {
        warehouse,
        robot_moves: input.robot_moves.clone(),
    });

    // GPS Coordinates
    let mut sum = 0;

    for (coord, c) in result.iter() {
        if c == &'[' {
            sum += 100 * coord.y + coord.x;
        }
    }

    sum.to_string()
}

pub fn can_push_box(grid: &mut Grid2D<char>, box_pos: Coordinate, dir: Direction) -> bool {
    if grid[box_pos] == '.' {
        return true;
    }

    if grid[box_pos] == '#' {
        return false;
    }

    match dir {
        Direction::Left | Direction::Right => {
            let next = box_pos + dir.into();
            return can_push_box(grid, next, dir);
        }

        Direction::Up | Direction::Down => {
            let next_pos1 = box_pos + dir.into();
            let next_pos2 = box_pos
                + dir.into()
                + if grid[box_pos] == '[' {
                    Direction::Right.into()
                } else {
                    Direction::Left.into()
                };

            return can_push_box(grid, next_pos1, dir) && can_push_box(grid, next_pos2, dir);
        }

        _ => false,
    }
}

pub fn push_box(grid: &mut Grid2D<char>, box_pos: Coordinate, dir: Direction) {
    if grid[box_pos] == '.' {
        return;
    }

    if grid[box_pos] == '#' {
        return;
    }

    if can_push_box(grid, box_pos, dir) {
        if dir == Direction::Left || dir == Direction::Right {
            let mut cur = box_pos;
            let mut next;

            loop {
                next = cur + dir.into();

                if grid[next] == '.' {
                    break;
                }

                cur = next;
            }

            while next != box_pos {
                grid.set(next, grid[cur]);
                grid.set(cur, '.');
                next = cur;
                cur = cur.neighbor(dir.opposite());
            }
        } else {
            let second_half_dir = if grid[box_pos] == '[' {
                Direction::Right
            } else {
                Direction::Left
            };

            let opposite_bracket = if grid[box_pos] == '[' { ']' } else { '[' };

            // Recursively push the boxes
            push_box(grid, box_pos + dir.into(), dir);
            push_box(grid, box_pos + dir.into() + second_half_dir.into(), dir);

            // Move the box itself
            grid.set(box_pos + dir.into(), grid[box_pos]);
            grid.set(
                box_pos + dir.into() + second_half_dir.into(),
                opposite_bracket,
            );
            grid.set(box_pos, '.');
            grid.set(box_pos + second_half_dir.into(), '.');
        }
    }
}

pub fn run_robot(input: &PuzzleInput) -> Grid2D<char> {
    let mut grid = input.warehouse.clone();
    let mut robot_pos = grid.iter().find(|(_, c)| **c == '@').unwrap().0;

    for robot_move in input.robot_moves.iter() {
        let dir: Direction = (*robot_move).try_into().unwrap();

        if can_push_box(&mut grid, robot_pos + dir.into(), dir) {
            push_box(&mut grid, robot_pos + dir.into(), dir);
            grid.set(robot_pos, '.');
            grid.set(robot_pos + dir.into(), '@');
            robot_pos = robot_pos + dir.into();
        }
    }

    grid
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::indoc;

    const TEST_INPUT: &str = indoc! {"
        ##########
        #..O..O.O#
        #......O.#
        #.OO..O.O#
        #..O@..O.#
        #O#..O...#
        #O..O..O.#
        #.OO.O.OO#
        #....O...#
        ##########

        <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
        vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
        ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
        <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
        ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
        ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
        >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
        <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
        ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
        v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
    "};

    #[test]
    fn test_part2_example() {
        let input = crate::parser::part2(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(part2(&input), "9021");
    }
}
