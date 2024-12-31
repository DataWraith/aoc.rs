use utility_belt::prelude::*;

use crate::parser::*;

pub fn part1(input: &PuzzleInput) -> String {
    input
        .lines
        .iter()
        .cloned()
        .filter(|(a, b)| a.x == b.x || a.y == b.y)
        .fold(HashMap::new(), |mut acc, (a, b)| {
            let (a, b) = bounding_box([a, b].into_iter());

            for x in a.x..=b.x {
                for y in a.y..=b.y {
                    acc.entry((x, y)).and_modify(|v| *v += 1).or_insert(1);
                }
            }

            acc
        })
        .iter()
        .filter(|(_, v)| **v >= 2)
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = indoc! {"
        0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2
    "};

    #[test]
    fn test_part1_example() {
        let input = crate::parser::part1(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(part1(&input), "5");
    }
}
