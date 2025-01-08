use utility_belt::prelude::*;

use crate::{
    p1::{addition, magnitude},
    parser::*,
};

pub fn part2(input: &PuzzleInput) -> String {
    let result = input
        .numbers
        .iter()
        .cloned()
        .permutations(2)
        .map(|numbers| addition(numbers[0].clone(), numbers[1].clone()))
        .map(|result| magnitude(result))
        .max()
        .unwrap();

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;
    use utility_belt::prelude::*;

    #[test]
    fn test_part2_example() {
        let input = parser::part2(parser::TEST_INPUT);
        assert_ne!(parser::TEST_INPUT.trim(), "TODO");
        assert_eq!(part2(&input), "3993");
    }
}
