use utility_belt::prelude::*;

use crate::parser::*;

pub fn part2(input: &PuzzleInput) -> String {
    input
        .lines
        .iter()
        .cloned()
        .fold(HashMap::new(), |mut acc, (a, b)| {
            let mut x = a.x;
            let mut y = a.y;

            let dx = if a.x != b.x { (b.x - a.x).signum() } else { 0 };
            let dy = if a.y != b.y { (b.y - a.y).signum() } else { 0 };

            // Go from a to b
            while x != b.x || y != b.y {
                acc.entry((x, y)).and_modify(|v| *v += 1).or_insert(1);

                x += dx;
                y += dy;
            }

            // Add the last point
            acc.entry((b.x, b.y)).and_modify(|v| *v += 1).or_insert(1);

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
    fn test_part2_example() {
        let input = crate::parser::part2(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(part2(&input), "12");
    }
}
