use crate::structs::*;

use utility_belt::prelude::*;

pub fn part1(input: &PuzzleInput) -> String {
    let mut flows = HashMap::default();
    let mut rating_sum = 0;

    for part in input.parts.iter() {
        let flow = Flow {
            part: part.clone(),
            current_workflow: "in".to_string(),
            current_index: 0,
        };

        flows.insert(flow, 1);
    }

    loop {
        let mut new_flows =
            utility_belt::misc::state_iteration(&flows, |f, _| transition(&input.workflows, f), ());

        for (flow, count) in new_flows.iter_mut() {
            if flow.current_workflow == "A" {
                rating_sum += flow.part.rating() * *count;
            }
        }

        new_flows.retain(|f, _| f.current_workflow != "A" && f.current_workflow != "R");

        if new_flows.is_empty() {
            break;
        }

        flows = new_flows;
    }

    rating_sum.to_string()
}

fn transition(workflows: &HashMap<String, Workflow>, from: &Flow) -> Vec<Flow> {
    let workflow = workflows.get(&from.current_workflow).unwrap();
    let rule = workflow.rules.get(from.current_index);

    if rule.is_none() {
        return vec![Flow {
            current_workflow: workflow.default.clone(),
            current_index: 0,
            ..from.clone()
        }];
    }

    let rule = rule.unwrap();

    let advance_current_workflow = vec![Flow {
        current_index: from.current_index + 1,
        ..from.clone()
    }];

    let next_workflow = vec![Flow {
        current_index: 0,
        current_workflow: rule.next.clone(),
        ..from.clone()
    }];

    use std::cmp::Ordering::*;

    match (
        rule.category,
        rule.comparison,
        from.part.x.cmp(&rule.value),
        from.part.m.cmp(&rule.value),
        from.part.a.cmp(&rule.value),
        from.part.s.cmp(&rule.value),
    ) {
        ('x', '<', Less, _, _, _) => next_workflow,
        ('x', '>', Greater, _, _, _) => next_workflow,
        ('m', '<', _, Less, _, _) => next_workflow,
        ('m', '>', _, Greater, _, _) => next_workflow,
        ('a', '<', _, _, Less, _) => next_workflow,
        ('a', '>', _, _, Greater, _) => next_workflow,
        ('s', '<', _, _, _, Less) => next_workflow,
        ('s', '>', _, _, _, Greater) => next_workflow,
        _ => advance_current_workflow,
    }
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
    fn test_part1() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part1(&input), "19114");
    }
}
