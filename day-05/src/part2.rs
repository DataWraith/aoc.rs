use std::collections::VecDeque;

use crate::structs::*;

pub fn part2(input: &PuzzleInput) -> String {
    let mut minimum = isize::MAX;

    for chunk in input.seeds.chunks(2) {
        let start = chunk[0];
        let length = chunk[1];

        let cur_best = lookup_best_seed(input, start, length);

        minimum = minimum.min(cur_best);
    }

    minimum.to_string()
}

fn lookup_best_seed(input: &PuzzleInput, start: isize, length: isize) -> isize {
    let mut q = VecDeque::new();
    let mut r = Vec::new();

    q.push_front((start..(start + length), 0, 0));

    while let Some((rng, map_id, entry_id)) = q.pop_front() {
        if map_id >= input.maps.len() {
            r.push(rng.start);
            continue;
        }

        let map = &input.maps[map_id];

        if entry_id >= map.len() {
            q.push_back((rng.clone(), map_id + 1, 0));
            continue;
        }

        let entry = &map[entry_id];
        let new_rngs = entry.range_split(rng.clone());

        for new_rng in new_rngs.iter() {
            if rng.clone().start == new_rng.start && rng.clone().end == new_rng.end {
                q.push_back((rng.clone(), map_id, entry_id + 1));
            } else {
                q.push_back((new_rng.clone(), map_id + 1, 0));
            }
        }
    }

    r.into_iter().min().unwrap_or(start)
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = indoc! {"
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4
    "};

    #[test]
    fn test_part2() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part2(&input), "46");
    }
}
