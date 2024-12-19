use std::iter::successors;

use utility_belt::prelude::*;

use crate::parser::*;

pub fn part2(input: &PuzzleInput) -> String {
    run_robot(input)
        .iter()
        .filter(|(_, &c)| c == '[')
        .map(|(coord, _)| 100 * coord.y + coord.x)
        .sum::<i32>()
        .to_string()
}

pub fn can_push_boxes(grid: &Grid2D<char>, box_pos: Coordinate, dir: Direction) -> bool {
    if grid[box_pos] == '#' {
        // Can't push into a wall
        return false;
    }

    if grid[box_pos] == '.' {
        // Can push into an empty space
        return true;
    }

    match dir {
        Direction::Left | Direction::Right => {
            // Scan left/right for the end of the line of boxes and return true
            // if we find an empty space
            for pos in successors(Some(box_pos), |&pos| Some(pos + dir)) {
                if grid[pos] == '#' {
                    return false;
                }

                if grid[pos] == '.' {
                    return true;
                }
            }

            unreachable!("Warehouse wall was incomplete");
        }

        Direction::Up | Direction::Down => {
            // Calculate where the currently considered half of the box would
            // end up if we were to push it
            let next_pos1 = box_pos + dir;

            // Depending on whether we're pushing the left or right half, the
            // other half will either be to the right or left of it.
            let next_pos2 = if grid[box_pos] == '[' {
                next_pos1 + Direction::Right
            } else {
                next_pos1 + Direction::Left
            };

            // Recursively check if the boxes that might occupy the two spaces
            // are pushable.
            can_push_boxes(grid, next_pos1, dir) && can_push_boxes(grid, next_pos2, dir)
        }

        _ => false,
    }
}

pub fn push_boxes(grid: &mut Grid2D<char>, box_pos: Coordinate, dir: Direction) {
    if grid[box_pos] == '.' {
        // Can't push something if the space is empty
        return;
    }

    let opposite_bracket = |c: char| if c == '[' { ']' } else { '[' };

    if dir == Direction::Left || dir == Direction::Right {
        // Scan left/right and push the boxes. Since they move exactly one space
        // at a time, we can just replace left brackets with right brackets and
        // vice versa.
        for pos in successors(Some(box_pos), |&pos| Some(pos + dir)) {
            if grid[pos] == '[' || grid[pos] == ']' {
                grid[pos] = opposite_bracket(grid[pos]);
            }

            // The empty space at the end of the line of boxes will always be
            // either a left half (if we're pushing from right to left) or a
            // right half (if we're pushing from left to right).
            if grid[pos] == '.' {
                if dir == Direction::Left {
                    grid[pos] = '[';
                } else {
                    grid[pos] = ']';
                }

                break;
            }
        }

        // Finally, the initial space can now be marked as empty
        grid[box_pos] = '.';

        return;
    }

    let dir_to_second_half = if grid[box_pos] == '[' {
        Direction::Right
    } else {
        Direction::Left
    };

    // Recursively push the adjacent boxes
    push_boxes(grid, box_pos + dir, dir);
    push_boxes(grid, box_pos + dir + dir_to_second_half, dir);

    // Move the box itself
    grid.set(box_pos + dir, grid[box_pos]);
    grid.set(
        box_pos + dir + dir_to_second_half,
        opposite_bracket(grid[box_pos]),
    );
    grid.set(box_pos, '.');
    grid.set(box_pos + dir_to_second_half, '.');
}

pub fn run_robot(input: &PuzzleInput) -> Grid2D<char> {
    let mut grid = input.warehouse.clone();

    // Find the robot's starting position and mark it as empty
    let mut robot_pos = grid.iter().find(|(_, c)| **c == '@').unwrap().0;
    grid.set(robot_pos, '.');

    for robot_move in input.robot_moves.iter() {
        let dir: Direction = (*robot_move).try_into().unwrap();

        if can_push_boxes(&grid, robot_pos + dir, dir) {
            push_boxes(&mut grid, robot_pos + dir, dir);
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
