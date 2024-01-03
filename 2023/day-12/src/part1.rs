use crate::structs::*;

pub fn part1(input: &PuzzleInput) -> String {
    input
        .records
        .iter()
        .map(|r| count_arrangements(&r.0, &r.1))
        .sum::<usize>()
        .to_string()
}

pub fn count_arrangements(input: &str, broken: &[usize]) -> usize {
    #[comemo::memoize]
    fn inner(input: &[char], broken: &[usize]) -> usize {
        if input.is_empty() {
            return broken.is_empty() as usize;
        }

        if broken.is_empty() {
            return !input.contains(&'#') as usize;
        }

        let mut count = 0;

        if input[0] == '.' || input[0] == '?' {
            count += inner(&input[1..], broken);
        }

        if input[0] == '#' || input[0] == '?' {
            let mut fits = true;

            fits = fits && broken[0] < input.len();
            fits = fits && input[0..broken[0]].iter().all(|c| *c != '.');
            fits = fits && input[broken[0]] != '#';

            if fits {
                count += inner(&input[(broken[0] + 1)..], &broken[1..]);
            }
        }

        count
    }

    let chars = std::iter::once('.')
        .chain(input.chars())
        .chain(std::iter::once('.'))
        .collect::<Vec<_>>();

    inner(&chars, broken)
}

#[cfg(test)]
mod tests {
    use super::*;

    use utility_belt::prelude::rstest;

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
