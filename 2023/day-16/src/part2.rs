use crate::{part1::solve, structs::*};

use utility_belt::prelude::*;

pub fn part2(input: &PuzzleInput) -> String {
    let mut max = 0;

    for x in 0..input.grid.width() {
        let downwards = Beam {
            position: Coordinate::new(x as i32, -1),
            direction: Direction::Down,
        };

        let upwards = Beam {
            position: Coordinate::new(x as i32, input.grid.height() as i32),
            direction: Direction::Up,
        };

        max = max.max(solve(input, downwards));
        max = max.max(solve(input, upwards));
    }

    for y in 0..input.grid.height() {
        let rightwards = Beam {
            position: Coordinate::new(-1, y as i32),
            direction: Direction::Right,
        };

        let leftwards = Beam {
            position: Coordinate::new(input.grid.width() as i32, y as i32),
            direction: Direction::Left,
        };

        max = max.max(solve(input, leftwards));
        max = max.max(solve(input, rightwards));
    }

    max.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::indoc;

    const TEST_INPUT: &str = indoc! {r#"
        .|...\....
        |.-.\.....
        .....|-...
        ........|.
        ..........
        .........\
        ..../.\\..
        .-.-/..|..
        .|....-|.\
        ..//.|....
    "#};

    #[test]
    fn test_part2() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part2(&input), "51");
    }
}
