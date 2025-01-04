use crate::parser::*;

pub fn part2(input: &PuzzleInput) -> String {
    evaluate_packet(&input.packets).to_string()
}

pub fn evaluate_packet(packet: &Packet) -> u64 {
    match packet.type_id {
        0 => packet.subpackets.iter().map(evaluate_packet).sum(),
        1 => packet.subpackets.iter().map(evaluate_packet).product(),
        2 => packet.subpackets.iter().map(evaluate_packet).min().unwrap(),
        3 => packet.subpackets.iter().map(evaluate_packet).max().unwrap(),
        4 => match packet.packet_type {
            PacketType::Literal(value) => value,
            _ => unreachable!("Invalid packet type: {:?}", packet.packet_type),
        },
        5 => {
            if evaluate_packet(&packet.subpackets[0]) > evaluate_packet(&packet.subpackets[1]) {
                1
            } else {
                0
            }
        }
        6 => {
            if evaluate_packet(&packet.subpackets[0]) < evaluate_packet(&packet.subpackets[1]) {
                1
            } else {
                0
            }
        }
        7 => {
            if evaluate_packet(&packet.subpackets[0]) == evaluate_packet(&packet.subpackets[1]) {
                1
            } else {
                0
            }
        }
        _ => unreachable!("Type ID has more than three bits: {}", packet.type_id),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;

    #[test]
    fn test_part2_example() {
        let input = parser::part2("9C0141080250320F1802104A08");
        assert_eq!(part2(&input), "1");
    }
}
