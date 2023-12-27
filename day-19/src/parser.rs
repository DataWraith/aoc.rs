use nom::{
    bytes::complete::{tag, take_until},
    character::complete::newline,
    combinator::eof,
    IResult,
};
use utility_belt::prelude::*;

use crate::structs::*;

fn nom_parser(input: &str) -> IResult<&str, PuzzleInput> {
    let (input, workflows) = parse_workflows(input)?;
    let (input, _) = newline(input)?;
    let (input, parts) = parse_parts(input)?;

    let (input, _) = eof(input)?;

    Ok((input, PuzzleInput { workflows, parts }))
}

pub fn parse(input: &str) -> PuzzleInput {
    nom_parser(input).unwrap().1
}

fn parse_parts(input: &str) -> IResult<&str, Vec<Part>> {
    let (input, parts) = nom::multi::many1(parse_part)(input)?;
    Ok((input, parts))
}

fn parse_part(input: &str) -> IResult<&str, Part> {
    let (input, _) = tag("{x=")(input)?;
    let (input, x) = parse_usize(input)?;
    let (input, _) = tag(",m=")(input)?;
    let (input, m) = parse_usize(input)?;
    let (input, _) = tag(",a=")(input)?;
    let (input, a) = parse_usize(input)?;
    let (input, _) = tag(",s=")(input)?;
    let (input, s) = parse_usize(input)?;
    let (input, _) = tag("}")(input)?;
    let (input, _) = newline(input)?;

    Ok((input, Part { x, m, a, s }))
}

fn parse_workflows(input: &str) -> IResult<&str, HashMap<String, Workflow>> {
    let (input, workflows) = nom::multi::many1(parse_workflow)(input)?;
    Ok((input, workflows.into_iter().collect()))
}

fn parse_workflow(input: &str) -> IResult<&str, (String, Workflow)> {
    let (input, name) = take_until("{")(input)?;
    let (input, _) = tag("{")(input)?;
    let (input, rules) = parse_rules(input)?;
    let (input, default) = parse_default(input)?;
    let (input, _) = newline(input)?;

    Ok((input, (name.to_string(), Workflow { rules, default })))
}

fn parse_rules(input: &str) -> IResult<&str, Vec<Rule>> {
    let (input, rules) = nom::multi::many1(parse_rule)(input)?;
    Ok((input, rules))
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    let (input, category) = nom::character::complete::one_of("xmas")(input)?;
    let (input, comparison) = nom::character::complete::one_of("<>")(input)?;
    let (input, value) = parse_usize(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, next) = take_until(",")(input)?;
    let (input, _) = tag(",")(input)?;

    Ok((
        input,
        Rule {
            category,
            comparison,
            value,
            next: next.to_string(),
        },
    ))
}

fn parse_default(input: &str) -> IResult<&str, String> {
    let (input, default) = take_until("}")(input)?;
    let (input, _) = tag("}")(input)?;

    Ok((input, default.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::indoc;

    const TEST_INPUT: &str = indoc! {"
        px{a<2006:qkq,m>2090:A,rfg}
        pv{a>1716:R,A}
        lnx{m>1548:A,A}
        rfg{s<537:gd,x>2440:R,A}
        qs{s>3448:A,lnx}
        qkq{x<1416:A,crn}
        crn{x>2662:A,R}
        in{s<1351:px,qqz}
        qqz{s>2770:qs,m<1801:hdj,R}
        gd{a>3333:R,R}
        hdj{m>838:A,pv}

        {x=787,m=2655,a=1222,s=2876}
        {x=1679,m=44,a=2067,s=496}
        {x=2036,m=264,a=79,s=2244}
        {x=2461,m=1339,a=466,s=291}
        {x=2127,m=1623,a=2188,s=1013}
    "};

    #[test]
    fn test_parse() {
        assert!(nom_parser(TEST_INPUT).is_ok());
    }
}
