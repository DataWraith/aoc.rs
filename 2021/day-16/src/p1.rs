use crate::parser::*;

pub fn part1(input: &PuzzleInput) -> String {
    let packet = Packet::read(&mut input.packets.clone());
    add_version_sum(&packet).to_string()
}

pub fn add_version_sum(packet: &Packet) -> u64 {
    packet.version as u64
        + packet
            .subpackets
            .iter()
            .map(|p| add_version_sum(p))
            .sum::<u64>()
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
