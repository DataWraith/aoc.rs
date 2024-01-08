use crate::structs::*;

use utility_belt::prelude::*;

pub fn part1(input: &PuzzleInput) -> String {
    load(&tilt(input, Direction::Up)).to_string()
}

pub fn load(input: &PuzzleInput) -> usize {
    let mut load = 0;

    input.grid.iter().for_each(|(coord, c)| {
        if *c == 'O' {
            load += input.grid.height() - coord.y() as usize;
        }
    });

    load
}

pub fn tilt(input: &PuzzleInput, direction: Direction) -> PuzzleInput {
    let mut grid = input.grid.clone();

    match direction {
        Direction::Up => {}
        Direction::Down => grid.mirror_y(),
        Direction::Left => grid.transpose(),
        Direction::Right => {
            grid.transpose();
            grid.mirror_y();
        }
    };

    loop {
        let mut changed = false;

        for y in 1..grid.height() {
            for x in 0..grid.width() {
                let c = Coordinate::new(x as i32, y as i32);
                let c2 = Coordinate::new(x as i32, (y - 1) as i32);

                if grid[c2] == '.' && grid[c] == 'O' {
                    grid[c] = '.';
                    grid[c2] = 'O';
                    changed = true;
                }
            }
        }

        if !changed {
            break;
        }
    }

    let mut g = grid.clone();

    match direction {
        Direction::Up => {}
        Direction::Down => g.mirror_y(),
        Direction::Left => g.transpose(),
        Direction::Right => {
            g.mirror_y();
            g.transpose();
        }
    }

    PuzzleInput { grid: g }
}

#[cfg(test)]
mod tests {
    use crate::parser::parse;

    use super::*;
    use utility_belt::prelude::indoc;

    const TEST_INPUT: &str = indoc! {"
        O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#....
    "};

    const TILTED: &str = indoc! {"
        OOOO.#.O..
        OO..#....#
        OO..O##..O
        O..#.OO...
        ........#.
        ..#....#.#
        ..O..#.O.O
        ..O.......
        #....###..
        #....#....
    "};

    #[test]
    fn test_part1() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part1(&input), "136");
    }

    #[test]
    fn test_tilt() {
        let initial = parse(TEST_INPUT);
        let expected = parse(TILTED);
        let processed = tilt(&initial, Direction::Up);
        assert_eq!(processed, expected);
    }
}
