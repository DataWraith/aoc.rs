use crate::structs::*;

pub fn part2(_input: &PuzzleInput) -> String {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    use utility_belt::prelude::indoc;

    const TEST_INPUT: &str = indoc! {"
        19, 13, 30 @ -2,  1, -2
        18, 19, 22 @ -1, -1, -2
        20, 25, 34 @ -2, -2, -4
        12, 31, 28 @ -1, -2, -1
        20, 19, 15 @  1, -5, -3
    "};

    #[test]
    fn test_part2() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part2(&input), "TODO");
    }
}
