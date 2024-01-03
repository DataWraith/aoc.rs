use utility_belt::prelude::*;

use crate::structs::*;

fn nom_parser(input: &str) -> IResult<&str, PuzzleInput> {
    let (input, blueprints) = many1(parse_blueprint)(input)?;
    let (input, _) = eof(input)?;

    Ok((input, PuzzleInput { blueprints }))
}

pub fn parse(input: &str) -> PuzzleInput {
    nom_parser(input).unwrap().1
}

fn parse_blueprint(input: &str) -> IResult<&str, Blueprint> {
    let (input, _) = tag("Blueprint ")(input)?;
    let (input, number) = digit1(input)?;
    let (input, _) = tag(": ")(input)?;

    let (input, _) = tag("Each ore robot costs ")(input)?;
    let (input, ore_robot_cost) = digit1(input)?;
    let (input, _) = tag(" ore. ")(input)?;

    let (input, _) = tag("Each clay robot costs ")(input)?;
    let (input, clay_robot_ore_cost) = digit1(input)?;
    let (input, _) = tag(" ore. ")(input)?;

    let (input, _) = tag("Each obsidian robot costs ")(input)?;
    let (input, obsidian_robot_ore_cost) = digit1(input)?;
    let (input, _) = tag(" ore and ")(input)?;
    let (input, obsidian_robot_clay_cost) = digit1(input)?;
    let (input, _) = tag(" clay. ")(input)?;

    let (input, _) = tag("Each geode robot costs ")(input)?;
    let (input, geode_robot_ore_cost) = digit1(input)?;
    let (input, _) = tag(" ore and ")(input)?;
    let (input, geode_robot_obsidian_cost) = digit1(input)?;
    let (input, _) = tag(" obsidian.")(input)?;
    let (input, _) = newline(input)?;

    let ore_robot_cost = Resources {
        ore: ore_robot_cost.parse().unwrap(),
        clay: 0,
        obsidian: 0,
        geodes: 0,
    };

    let clay_robot_cost = Resources {
        ore: clay_robot_ore_cost.parse().unwrap(),
        clay: 0,
        obsidian: 0,
        geodes: 0,
    };

    let obsidian_robot_cost = Resources {
        ore: obsidian_robot_ore_cost.parse().unwrap(),
        clay: obsidian_robot_clay_cost.parse().unwrap(),
        obsidian: 0,
        geodes: 0,
    };

    let geode_robot_cost = Resources {
        ore: geode_robot_ore_cost.parse().unwrap(),
        obsidian: geode_robot_obsidian_cost.parse().unwrap(),
        clay: 0,
        geodes: 0,
    };

    let max_resource = [
        ore_robot_cost.clone(),
        clay_robot_cost.clone(),
        obsidian_robot_cost.clone(),
        geode_robot_cost.clone(),
    ]
    .iter()
    .cloned()
    .reduce(|acc, cost| Resources {
        ore: acc.ore.max(cost.ore),
        clay: acc.clay.max(cost.clay),
        obsidian: acc.obsidian.max(cost.obsidian),
        geodes: i16::MAX,
    })
    .unwrap();

    Ok((
        input,
        Blueprint {
            number: number.parse().unwrap(),
            robot_costs: [
                ore_robot_cost,
                clay_robot_cost,
                obsidian_robot_cost,
                geode_robot_cost,
            ],
            max_resources: max_resource,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_parse() {
        assert!(nom_parser(TEST_INPUT).is_ok());
    }
}
