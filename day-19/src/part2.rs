use std::collections::BTreeMap;

use crate::structs::*;

use utility_belt::prelude::*;

pub fn part2(input: &PuzzleInput) -> String {
    let mut flows = HashMap::default();
    let mut accepted = Vec::new();

    let mut initial_part_range = BTreeMap::new();
    initial_part_range.insert('x', 1..4001);
    initial_part_range.insert('m', 1..4001);
    initial_part_range.insert('a', 1..4001);
    initial_part_range.insert('s', 1..4001);

    let flow = RangeFlow {
        part: initial_part_range,
        current_workflow: "in".to_string(),
        current_index: 0,
        accepted: 4000 * 4000 * 4000 * 4000,
    };

    flows.insert(flow, 1);

    loop {
        let mut new_flows =
            utility_belt::misc::state_iteration(&flows, |f, _| transition(&input.workflows, f), ());

        for (flow, _count) in new_flows.iter_mut() {
            if flow.current_workflow == "A" {
                accepted.push(flow.clone());
            }
        }

        new_flows.retain(|f, _| f.current_workflow != "A" && f.current_workflow != "R");

        if new_flows.is_empty() {
            break;
        }

        flows = new_flows;
    }

    accepted
        .iter()
        .map(|x| x.accepted)
        .sum::<usize>()
        .to_string()
}

fn transition(workflows: &HashMap<String, Workflow>, from: &RangeFlow) -> Vec<RangeFlow> {
    let mut result = Vec::default();

    let workflow = workflows.get(&from.current_workflow).unwrap();
    let rule = workflow.rules.get(from.current_index);

    if rule.is_none() {
        let mut new_flow = from.clone();
        new_flow.current_workflow = workflow.default.clone();
        new_flow.current_index = 0;
        result.push(new_flow);
        return result;
    }

    let rule = rule.unwrap();

    let ((lower_next, lower_next_index), (upper_next, upper_next_index)) = match rule.comparison {
        '<' => (
            (rule.next.clone(), 0),
            (from.current_workflow.clone(), from.current_index + 1),
        ),
        '>' => (
            (from.current_workflow.clone(), from.current_index + 1),
            (rule.next.clone(), 0),
        ),
        _ => unreachable!("Invalid comparison"),
    };

    let (lower_range, upper_range) = match rule.comparison {
        '<' => (
            from.part[&rule.category].start..rule.value,
            rule.value..from.part[&rule.category].end,
        ),

        '>' => (
            from.part[&rule.category].start..(rule.value + 1),
            (rule.value + 1)..from.part[&rule.category].end,
        ),

        _ => unreachable!("Invalid comparison"),
    };

    let mut part_range_lower = from.part.clone();
    part_range_lower.insert(rule.category, lower_range.clone());

    let mut part_range_upper = from.part.clone();
    part_range_upper.insert(rule.category, upper_range.clone());

    let full_count = from.part[&rule.category].end - from.part[&rule.category].start;

    if !lower_range.is_empty() {
        let lower_count = lower_range.end - lower_range.start;

        result.push(RangeFlow {
            part: part_range_lower,
            current_workflow: lower_next,
            current_index: lower_next_index,
            accepted: from.accepted * lower_count / full_count,
        });
    };

    if !upper_range.is_empty() {
        let upper_count = upper_range.end - upper_range.start;

        result.push(RangeFlow {
            part: part_range_upper,
            current_workflow: upper_next,
            current_index: upper_next_index,
            accepted: from.accepted * upper_count / full_count,
        });
    };

    result
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_part2() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part2(&input), "167409079868000");
    }
}
