use crate::structs::*;

use utility_belt::prelude::*;

pub fn part1(input: &PuzzleInput) -> String {
    solve(input, 0)
}

pub fn solve(input: &PuzzleInput, max_discrepancies: usize) -> String {
    let horizontal = input
        .patterns
        .iter()
        .flat_map(|p| find_reflections(&p.transpose(), max_discrepancies))
        .map(|y| 100 * (y + 1))
        .sum::<usize>();

    let vertical = input
        .patterns
        .iter()
        .flat_map(|p| find_reflections(p, max_discrepancies))
        .map(|x| x + 1)
        .sum::<usize>();

    (vertical + horizontal).to_string()
}

pub fn find_reflections(pattern: &MirrorPattern, max_discrepancies: usize) -> Vec<usize> {
    let mut candidates = Vec::new();
    let mut result = Vec::new();

    for range in [(0..pattern.width()), (1..pattern.width())] {
        for [x1, x2] in range.array_chunks() {
            let mut discrepancies = 0;

            for y in 0..pattern.height() {
                let c1 = Coordinate::new(x1 as i32, y as i32);
                let c2 = Coordinate::new(x2 as i32, y as i32);

                if pattern[c1] != pattern[c2] {
                    discrepancies += 1;

                    if discrepancies > max_discrepancies {
                        break;
                    }
                }

                if y == pattern.height() - 1 {
                    candidates.push(x1);
                }
            }
        }
    }

    for c in candidates.into_iter() {
        let indices = reflection_indices(c, pattern.width());

        if indices
            .into_iter()
            .map(|(x1, x2)| check_reflection(pattern, (x1, x2), max_discrepancies))
            .sum::<usize>()
            == max_discrepancies
        {
            result.push(c);
        }
    }

    result
}

pub fn check_reflection(
    pattern: &MirrorPattern,
    pair: (usize, usize),
    max_discrepancies: usize,
) -> usize {
    let (x1, x2) = pair;

    let mut discrepancies = 0;

    for y in 0..pattern.height() {
        let c1 = Coordinate::new(x1 as i32, y as i32);
        let c2 = Coordinate::new(x2 as i32, y as i32);

        if pattern[c1] != pattern[c2] {
            discrepancies += 1;

            if discrepancies > max_discrepancies {
                return discrepancies;
            }
        }
    }

    discrepancies
}

pub fn reflection_indices(index: usize, size: usize) -> Vec<(usize, usize)> {
    let mut result = Vec::new();

    let mut r1 = index as isize;
    let mut r2 = index as isize + 1;

    while r1 >= 0 && r2 < size as isize {
        result.push((r1 as usize, r2 as usize));

        r1 -= 1;
        r2 += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::indoc;

    const TEST_INPUT: &str = indoc! {"
        #.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.

        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#
    "};

    #[test]
    fn test_part1() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part1(&input), "405");
    }
}
