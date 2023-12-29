mod parser;
mod part1;
mod part2;
mod structs;

use utility_belt::prelude::*;

const INPUT1: &str = indoc! {"
    Time:        58     99     64     69
    Distance:   478   2232   1019   1071
"};

const INPUT2: &str = indoc! {"
    Time:        58996469
    Distance:   478223210191071
"};

fn main() {
    let input1 = parser::parse(INPUT1);
    let input2 = parser::parse(INPUT2);
    println!("Part 1: {}", part1::part1(&input1));
    println!("Part 2: {}", part2::part2(&input2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let puzzle_input = parser::parse(INPUT1);
        assert_eq!(part1::part1(&puzzle_input), "128700");
    }

    #[test]
    fn test_part2() {
        let puzzle_input = parser::parse(INPUT2);
        assert_eq!(part2::part2(&puzzle_input), "39594072");
    }
}
