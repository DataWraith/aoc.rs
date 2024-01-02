use crate::structs::*;

use utility_belt::prelude::*;

pub fn part2(input: &PuzzleInput) -> String {
    let mut screen = Grid2D::new(40, 6, '.');

    let xs = input
        .instructions
        .iter()
        .scan((0, 1), |mut acc, (cycles, delta)| {
            acc.0 += cycles;
            acc.1 += delta;
            Some(acc.clone())
        })
        .collect::<Vec<_>>();

    let mut last_cycle = 0;
    let mut unrolled = xs
        .iter()
        .flat_map(|(cycles, delta)| {
            let delta_cycles = cycles - last_cycle;
            last_cycle = *cycles;

            let mut v = Vec::new();

            for _ in 0..delta_cycles {
                v.push(*delta);
            }

            v
        })
        .collect::<Vec<_>>();

    unrolled.insert(0, 1);
    unrolled.insert(0, 1);
    dbg!(&unrolled[0..10]);

    let mut cur = Coordinate::new(0, 0);

    for (i, register) in unrolled.iter().enumerate().take(40 * 6) {
        let x = i % 40;

        dbg!(cur, register, x, i);

        if (x as isize).abs_diff(*register) < 2 {
            dbg!("DRAW");
            screen[cur] = '#';
        } else {
            screen[cur] = '.';
        }

        cur += Direction::Right.into();

        if cur.x() >= screen.width() as i32 {
            cur = Coordinate::new(0, cur.y() + 1);
        }

        if cur.x() > 3 {
            //break;
        }
    }

    format!("{}", screen)
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = include_str!("../test.txt");
    const EXPECTED: &str = include_str!("../expected.txt");

    #[test]
    fn test_part2() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part2(&input), "TODO");
    }
}
