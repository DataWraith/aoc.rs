use crate::{p1::first_illegal_character, parser::*};

pub fn part2(input: &PuzzleInput) -> String {
    let mut scores = input
        .lines
        .iter()
        .cloned()
        .filter(|line| first_illegal_character(line).is_none())
        .map(completion_score)
        .collect::<Vec<_>>();

    let middle_idx = scores.len() / 2;

    let (_, middle, _) = scores.select_nth_unstable(middle_idx);

    middle.to_string()
}

fn completion_score(line: &str) -> usize {
    let mut stack = Vec::new();
    let mut score = 0;

    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' | ']' | '}' | '>' => {
                let _ = stack.pop().unwrap();
            }
            _ => unreachable!(),
        }
    }

    while let Some(c) = stack.pop() {
        score = score * 5
            + match c {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => unreachable!(),
            };
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = indoc! {"
        [({(<(())[]>[[{[]{<()<>>
        [(()[<>])]({[<{<<[]>>(
        {([(<{}[<>[]}>{[]{[(<()>
        (((({<>}<{<{<>}{[]{[]{}
        [[<[([]))<([[{}[[()]]]
        [{[{({}]{}}([{[{{{}}([]
        {<[[]]>}<{[{[{[]{()[[[]
        [<(<(<(<{}))><([]([]()
        <{([([[(<>()){}]>(<<{{
        <{([{{}}[<[[[<>{}]]]>[]]
    "};

    #[test]
    fn test_part2_example() {
        let input = crate::parser::part2(TEST_INPUT);
        assert_ne!(TEST_INPUT.trim(), "TODO");
        assert_eq!(part2(&input), "288957");
    }
}
