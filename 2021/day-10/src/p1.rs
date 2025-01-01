use utility_belt::prelude::*;

use crate::parser::*;

pub fn part1(input: &PuzzleInput) -> String {
    input
        .lines
        .iter()
        .cloned()
        .filter_map(first_illegal_character)
        .map(|(c, top, i)| match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => 0,
        })
        .sum::<usize>()
        .to_string()
}

pub fn first_illegal_character(line: &str) -> Option<(char, char, usize)> {
    let mut stack = Vec::new();

    for (i, c) in line.chars().enumerate() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' | ']' | '}' | '>' => {
                let top = stack.pop().unwrap();

                match (top, c) {
                    ('(', ')') | ('[', ']') | ('{', '}') | ('<', '>') => (),
                    _ => return Some((c, top, i)),
                }
            }

            _ => unreachable!(),
        }
    }

    None
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
    fn test_part1_example() {
        let input = crate::parser::part1(TEST_INPUT);
        assert_ne!(TEST_INPUT.trim(), "TODO");
        assert_eq!(part1(&input), "26397");
    }
}
