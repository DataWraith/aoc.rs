use crate::structs::*;

use utility_belt::{misc::state_iteration, prelude::*};

pub fn part1(input: &PuzzleInput) -> String {
    input
        .records
        .iter()
        .map(|r| count_arrangements(&r.0, &r.1))
        .sum::<usize>()
        .to_string()
}

pub fn count_arrangements(input: &str, broken: &[usize]) -> usize {
    let automaton = Automaton::new(broken);

    let start = State::ColumnStart(0);

    let mut states = HashMap::default();
    states.insert(start, 1);

    let states = input
        .chars()
        .chain(vec!['.', 'x'])
        .fold(states, |states, c| {
            state_iteration(&states, |s, c| automaton.transition(s, *c), c)
        });

    states.get(&State::Final).copied().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    use utility_belt::prelude::rstest::*;

    #[rstest]
    #[case("???.### 1,1,3", 1)]
    #[case(".??..??...?##. 1,1,3", 4)]
    #[case("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
    #[case("????.#...#... 4,1,1", 1)]
    #[case("????.######..#####. 1,6,5", 4)]
    #[case("?###???????? 3,2,1", 10)]
    fn test_part1(#[case] input: &str, #[case] expected: usize) {
        let input = crate::parser::parse(input);
        assert_eq!(part1(&input), expected.to_string());
    }
}
