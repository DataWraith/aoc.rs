use crate::{parser, structs::*};

pub fn part2(input: &PuzzleInput) -> String {
    let packets = input
        .packets
        .iter()
        .cloned()
        .flat_map(|(a, b)| vec![a, b])
        .collect::<Vec<_>>();

    let dividers = parser::parse("[[2]]\n[[6]]\n\n");

    // We don't actually need to sort here, we just need to keep track of how
    // many packets are less than the dividers to compute the indices, making
    // this O(n) instead of O(n log n).
    let mut divider2_idx = 1;
    let mut divider6_idx = 2;

    for packet in packets.into_iter() {
        if packet < dividers.packets[0].0 {
            divider2_idx += 1;
        }

        if packet < dividers.packets[0].1 {
            divider6_idx += 1;
        }
    }

    (divider2_idx * divider6_idx).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part2() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part2(&input), "140");
    }
}
