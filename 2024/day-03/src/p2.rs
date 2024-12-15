use crate::structs::*;

pub fn part2(input: &PuzzleInput) -> String {
    input.muls.iter().sum::<usize>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = indoc! {"
        xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
    "};

    #[test]
    fn test_part2() {
        let input = crate::parser::parse2(TEST_INPUT);
        assert_eq!(part2(&input), "48");
    }
}
