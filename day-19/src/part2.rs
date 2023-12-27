use crate::structs::*;

use utility_belt::prelude::*;

pub fn part2(input: &PuzzleInput) -> String {
    let mut flows = HashMap::default();
    let mut accepted = Vec::new();

    let flow = RangeFlow {
        part: PartRange {
            x: 1..4001,
            m: 1..4001,
            a: 1..4001,
            s: 1..4001,
        },
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

    let (lower_range, upper_range) = match (rule.category, rule.comparison) {
        ('x', '<') => (from.part.x.start..rule.value, rule.value..from.part.x.end),
        ('x', '>') => (
            from.part.x.start..(rule.value + 1),
            (rule.value + 1)..from.part.x.end,
        ),
        ('m', '<') => (from.part.m.start..rule.value, rule.value..from.part.m.end),
        ('m', '>') => (
            from.part.m.start..(rule.value + 1),
            (rule.value + 1)..from.part.m.end,
        ),
        ('a', '<') => (from.part.a.start..rule.value, rule.value..from.part.a.end),
        ('a', '>') => (
            from.part.a.start..(rule.value + 1),
            (rule.value + 1)..from.part.a.end,
        ),
        ('s', '<') => (from.part.s.start..rule.value, rule.value..from.part.s.end),
        ('s', '>') => (
            from.part.s.start..(rule.value + 1),
            (rule.value + 1)..from.part.s.end,
        ),

        _ => unreachable!("Invalid comparison/comparison"),
    };

    let (part_range_lower, part_range_upper) = match rule.category {
        'x' => (
            PartRange {
                x: lower_range.clone(),
                ..from.part.clone()
            },
            PartRange {
                x: upper_range.clone(),
                ..from.part.clone()
            },
        ),
        'm' => (
            PartRange {
                m: lower_range.clone(),
                ..from.part.clone()
            },
            PartRange {
                m: upper_range.clone(),
                ..from.part.clone()
            },
        ),
        'a' => (
            PartRange {
                a: lower_range.clone(),
                ..from.part.clone()
            },
            PartRange {
                a: upper_range.clone(),
                ..from.part.clone()
            },
        ),
        's' => (
            PartRange {
                s: lower_range.clone(),
                ..from.part.clone()
            },
            PartRange {
                s: upper_range.clone(),
                ..from.part.clone()
            },
        ),
        _ => unreachable!("Invalid category"),
    };

    let full_count = match rule.category {
        'x' => from.part.x.end - from.part.x.start,
        'm' => from.part.m.end - from.part.m.start,
        'a' => from.part.a.end - from.part.a.start,
        's' => from.part.s.end - from.part.s.start,

        _ => unreachable!("Invalid category"),
    };

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
