use crate::{parser, structs::*};

use utility_belt::prelude::*;

pub fn part2(input: &PuzzleInput) -> String {
    let mut packets = input
        .packets
        .iter()
        .cloned()
        .flat_map(|(a, b)| vec![a, b])
        .collect::<Vec<_>>();

    let dividers = parser::parse("[[2]]\n[[6]]\n\n");

    packets.push(dividers.packets[0].0.clone());
    packets.push(dividers.packets[0].1.clone());

    packets.sort();

    packets
        .into_iter()
        .enumerate()
        .filter(|(i, p)| *p == dividers.packets[0].0 || *p == dividers.packets[0].1)
        .map(|(i, _)| i + 1)
        .product::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part2() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part2(&input), "140");
    }
}
