use crate::parser::*;

pub fn part1(input: &PuzzleInput) -> String {
    add_version_sum(&input.packets).to_string()
}

pub fn add_version_sum(packet: &Packet) -> u64 {
    packet.version as u64
        + match &packet.contents {
            PacketType::Literal(_) => 0,
            PacketType::Operator(subpackets) => subpackets.iter().map(add_version_sum).sum::<u64>(),
        }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;

    #[test]
    fn test_part1_example() {
        let input = parser::part1(parser::TEST_INPUT3);
        assert_ne!(parser::TEST_INPUT3.trim(), "TODO");
        assert_eq!(part1(&input), "31");
    }
}
