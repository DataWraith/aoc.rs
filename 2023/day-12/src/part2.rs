use crate::{part1::count_arrangements, structs::*};

use utility_belt::prelude::*;

pub fn part2(input: &PuzzleInput) -> String {
    let mut sum = 0;

    for (line, broken) in input.records.iter() {
        let repeated_line = [line].repeat(5).iter().join("?");
        let repeated_broken = broken.repeat(5);

        sum += count_arrangements(&repeated_line, &repeated_broken);
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    use utility_belt::prelude::rstest;

    #[rstest]
    #[case("???.### 1,1,3", 1)]
    #[case(".??..??...?##. 1,1,3", 16384)]
    #[case("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
    #[case("????.#...#... 4,1,1", 16)]
    #[case("????.######..#####. 1,6,5", 2500)]
    #[case("?###???????? 3,2,1", 506250)]
    fn test_part2(#[case] input: &str, #[case] expected: usize) {
        let input = crate::parser::parse(input);
        assert_eq!(part2(&input), expected.to_string());
    }
}
