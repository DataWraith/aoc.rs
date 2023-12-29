use crate::structs::*;

pub fn part1(input: &PuzzleInput) -> String {
    input
        .seeds
        .iter()
        .cloned()
        .map(|seed| lookup_seed_location(input, seed))
        .min()
        .unwrap()
        .to_string()
}

fn lookup_seed_location(input: &PuzzleInput, seed: isize) -> isize {
    let mut location_old = seed;
    let mut location = location_old;

    for entries in input.maps.iter() {
        for range_map in entries.iter() {
            location = range_map.lookup(location);
            if location != location_old {
                location_old = location;
                break;
            }

            location_old = location;
        }
    }

    location
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
    fn test_part1() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part1(&input), "35");
    }
}
