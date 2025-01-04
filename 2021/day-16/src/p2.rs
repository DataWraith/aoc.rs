use crate::parser::*;

pub fn part2(input: &PuzzleInput) -> String {
    let packet = Packet::read(&mut input.packets.clone());
    evaluate_packet(&packet).to_string()
}

pub fn evaluate_packet(packet: &Packet) -> u64 {
    match packet.type_id {
        0 => packet.subpackets.iter().map(|p| evaluate_packet(p)).sum(),
        1 => packet
            .subpackets
            .iter()
            .map(|p| evaluate_packet(p))
            .product(),
        2 => packet
            .subpackets
            .iter()
            .map(|p| evaluate_packet(p))
            .min()
            .unwrap(),
        3 => packet
            .subpackets
            .iter()
            .map(|p| evaluate_packet(p))
            .max()
            .unwrap(),
        4 => match packet.value {
            PacketType::Literal(value) => value,
            _ => unreachable!("Invalid packet type: {:?}", packet.value),
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
        _ => unreachable!("Invalid type id: {}", packet.type_id),
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
