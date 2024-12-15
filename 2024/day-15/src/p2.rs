use utility_belt::prelude::*;

use crate::parser::*;

#[tracing::instrument(skip(input))]
pub fn part2(input: &PuzzleInput) -> String {
    let result = run_robot(input);

    // GPS Coordinates
    let mut sum = 0;

    for (coord, c) in result.iter() {
        if c == &'[' {
            sum += 100 * coord.y + coord.x;
        }
    }

    sum.to_string()
}

pub fn can_push_boxes(grid: &Grid2D<char>, box_pos: Coordinate, dir: Direction) -> bool {
    if grid[box_pos] == '.' {
        return true;
    }

    if grid[box_pos] == '#' {
        return false;
    }

    match dir {
        Direction::Left | Direction::Right => {
            let next = box_pos + dir;
            can_push_boxes(grid, next, dir)
        }

        Direction::Up | Direction::Down => {
            let next_pos1 = box_pos + dir;
            let next_pos2 = box_pos
                + dir
                + if grid[box_pos] == '[' {
                    Direction::Right
                } else {
                    Direction::Left
                };

            can_push_boxes(grid, next_pos1, dir) && can_push_boxes(grid, next_pos2, dir)
        }

        _ => false,
    }
}

pub fn push_boxes(grid: &mut Grid2D<char>, box_pos: Coordinate, dir: Direction) {
    if grid[box_pos] == '.' {
        return;
    }

    if dir == Direction::Left || dir == Direction::Right {
        let mut cur = box_pos;
        let mut next;

        loop {
            next = cur + dir;

            if grid[next] == '.' {
                break;
            }

            cur = next;
        }

        while next != box_pos {
            grid.set(next, grid[cur]);
            next = cur;
            cur = cur.neighbor(dir.opposite());
        }

        grid.set(next, '.');
        return;
    }

    let second_half_dir = if grid[box_pos] == '[' {
        Direction::Right
    } else {
        Direction::Left
    };

    let opposite_bracket = if grid[box_pos] == '[' { ']' } else { '[' };

    // Recursively push the boxes
    push_boxes(grid, box_pos + dir, dir);
    push_boxes(grid, box_pos + dir + second_half_dir, dir);

    // Move the box itself
    grid.set(box_pos + dir, grid[box_pos]);
    grid.set(box_pos + dir + second_half_dir, opposite_bracket);
    grid.set(box_pos, '.');
    grid.set(box_pos + second_half_dir, '.');
}

pub fn run_robot(input: &PuzzleInput) -> Grid2D<char> {
    let mut grid = input.warehouse.clone();
    let mut robot_pos = grid.iter().find(|(_, c)| **c == '@').unwrap().0;

    for robot_move in input.robot_moves.iter() {
        let dir: Direction = (*robot_move).try_into().unwrap();

        if can_push_boxes(&grid, robot_pos + dir, dir) {
            push_boxes(&mut grid, robot_pos + dir, dir);
            grid.set(robot_pos, '.');
            grid.set(robot_pos + dir, '@');
            robot_pos += dir;
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
