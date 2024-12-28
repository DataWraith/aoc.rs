use crate::{p1::CircularList, parser::*};

pub fn part2(input: &PuzzleInput) -> String {
    let seq = input
        .sequence
        .iter()
        .map(|&x| x * 811589153)
        .collect::<Vec<_>>();
    let mut list = CircularList::new(&seq);

    for _ in 0..10 {
        list.mix(&seq);
    }

    let mut sum = 0;
    for i in 1..=3 {
        sum += list.get(1000 * i);
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = utility_belt::prelude::indoc! {"
        1
        2
        -3
        3
        -2
        0
        4
    "};

    #[test]
    fn test_part2_example() {
        let input = crate::parser::part2(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(part2(&input), "1623178306");
    }
}
