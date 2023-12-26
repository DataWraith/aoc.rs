use crate::structs::*;

pub fn part1(input: &PuzzleInput) -> String {
    (input.path_to_end("AAA".to_string()).count() - 1).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT1: &str = indoc! {"
        RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)
    "};

    #[test]
    fn test_part1() {
        let input = crate::parser::parse(TEST_INPUT1);
        assert_eq!(part1(&input), "2");
    }

    const TEST_INPUT2: &str = indoc! {"
        LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)
    "};

    #[test]
    fn test_part1_2() {
        let input = crate::parser::parse(TEST_INPUT2);
        assert_eq!(part1(&input), "6");
    }
}
