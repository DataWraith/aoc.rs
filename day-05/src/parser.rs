use nom::{
    bytes::complete::tag,
    character::complete::{newline, space1},
    combinator::eof,
    multi::{many0, many1, separated_list1},
    IResult,
};

use utility_belt::prelude::*;

use crate::structs::*;

pub fn nom_parser(input: &str) -> IResult<&str, PuzzleInput> {
    let (input, seeds) = parse_seeds(input)?;
    let (input, _) = tag("seed-to-soil map:\n")(input)?;
    let (input, seed_to_soil_map) = parse_map(input)?;
    let (input, _) = tag("soil-to-fertilizer map:\n")(input)?;
    let (input, soil_to_fertilizer_map) = parse_map(input)?;
    let (input, _) = tag("fertilizer-to-water map:\n")(input)?;
    let (input, fertilizer_to_water_map) = parse_map(input)?;
    let (input, _) = tag("water-to-light map:\n")(input)?;
    let (input, water_to_light_map) = parse_map(input)?;
    let (input, _) = tag("light-to-temperature map:\n")(input)?;
    let (input, light_to_temperature_map) = parse_map(input)?;
    let (input, _) = tag("temperature-to-humidity map:\n")(input)?;
    let (input, temperature_to_humidity_map) = parse_map(input)?;
    let (input, _) = tag("humidity-to-location map:\n")(input)?;
    let (input, humidity_to_location_map) = parse_map(input)?;
    let (input, _) = eof(input)?;

    Ok((
        input,
        PuzzleInput {
            seeds,
            maps: vec![
                seed_to_soil_map,
                soil_to_fertilizer_map,
                fertilizer_to_water_map,
                water_to_light_map,
                light_to_temperature_map,
                temperature_to_humidity_map,
                humidity_to_location_map,
            ],
        },
    ))
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<isize>> {
    let (input, _) = tag("seeds: ")(input)?;
    let (input, seeds) = separated_list1(space1, parse_isize)(input)?;
    let (input, _) = many1(newline)(input)?;

    Ok((input, seeds))
}

fn parse_map(input: &str) -> IResult<&str, Vec<RangeMap>> {
    let (input, map) = separated_list1(newline, parse_range)(input)?;
    let (input, _) = many0(newline)(input)?;

    Ok((input, map))
}

fn parse_range(input: &str) -> IResult<&str, RangeMap> {
    let (input, destination_range_start) = parse_isize(input)?;
    let (input, _) = space1(input)?;
    let (input, source_range_start) = parse_isize(input)?;
    let (input, _) = space1(input)?;
    let (input, range_length) = parse_isize(input)?;

    Ok((
        input,
        RangeMap {
            source: source_range_start..(source_range_start + range_length),
            destination: destination_range_start..(destination_range_start + range_length),
        },
    ))
}

pub fn parse(input: &str) -> PuzzleInput {
    nom_parser(input).unwrap().1
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::indoc;

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
    fn test_parse() {
        assert!(nom_parser(TEST_INPUT).is_ok());
    }
}
