use bitreader::BitReader;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum PacketType {
    Literal(u64),
    OperatorLength(u64),
    OperatorSubpackets(u64),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Packet {
    pub version: u8,
    pub type_id: u8,
    pub packet_type: PacketType,
    pub subpackets: Vec<Packet>,
}

fn read_literal_value(input: &mut BitReader, ans: &mut Vec<u8>) {
    let v = input.read_u8(5).unwrap();

    ans.push(v & 0b1111);

    if v & 0b10000 != 0 {
        read_literal_value(input, ans);
    }
}

fn convert_literal_value(nibbles: &[u8]) -> PacketType {
    let mut value = 0u64;

    assert!(nibbles.len() <= 64 / 4);

    for b in nibbles.iter() {
        assert!(*b < 16);
        value = (value << 4) | *b as u64;
    }

    PacketType::Literal(value)
}

fn read_operator_packet_length(input: &mut BitReader) -> PacketType {
    let length_type_id = input.read_bool().unwrap();

    if length_type_id {
        let length = input.read_64(11).unwrap();
        PacketType::OperatorSubpackets(length)
    } else {
        let length = input.read_u64(15).unwrap();
        PacketType::OperatorLength(length)
    }
}

impl Packet {
    pub fn read(input: &mut BitReader) -> Self {
        let version = input.read_u8(3).unwrap();
        let type_id = input.read_u8(3).unwrap();

        if type_id == 4 {
            let mut ans = vec![];
            read_literal_value(input, &mut ans);

            return Self {
                version,
                type_id,
                packet_type: convert_literal_value(&ans),
                subpackets: vec![],
            };
        }

        match read_operator_packet_length(input) {
            PacketType::Literal(_) => unreachable!("We already checked for literals"),

            PacketType::OperatorLength(length) => {
                let mut subpacket_data = input.relative_reader_atmost(length as u64);
                input.skip(length as u64).unwrap();

                let mut subpackets = vec![];

                while subpacket_data.remaining() > 0 {
                    subpackets.push(Packet::read(&mut subpacket_data));
                }

                Self {
                    version,
                    type_id,
                    packet_type: PacketType::OperatorLength(length),
                    subpackets,
                }
            }

            PacketType::OperatorSubpackets(length) => {
                let mut subpackets = vec![];

                for _ in 0..length {
                    subpackets.push(Packet::read(input));
                }

                Self {
                    version,
                    type_id,
                    packet_type: PacketType::OperatorSubpackets(length),
                    subpackets,
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub packets: Packet,
}

pub fn part1(input: &'static str) -> PuzzleInput {
    let mut bits = Vec::new();

    input.trim().as_bytes().chunks(2).for_each(|chunk| {
        let a = (chunk[0] as char).to_digit(16).unwrap() as u8;
        let b = (chunk[1] as char).to_digit(16).unwrap() as u8;

        bits.push((a << 4) | b);
    });

    let mut bits = BitReader::new(&bits);

    PuzzleInput {
        packets: Packet::read(&mut bits),
    }
}

pub fn part2(input: &'static str) -> PuzzleInput {
    part1(input)
}

pub const TEST_INPUT1: &str = "D2FE28";
pub const TEST_INPUT2: &str = "38006F45291200";
pub const TEST_INPUT3: &str = "A0016C880162017C3686B18A3D4780";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_example1() {
        let input = part1(TEST_INPUT1);
        assert_eq!(
            input.packets,
            Packet {
                version: 6,
                type_id: 4,
                packet_type: PacketType::Literal(2021),
                subpackets: vec![],
            }
        );
    }

    #[test]
    fn test_parse_example2() {
        let input = part1(TEST_INPUT2);
        assert_eq!(
            input.packets,
            Packet {
                version: 1,
                type_id: 6,
                packet_type: PacketType::OperatorLength(27),
                subpackets: vec![
                    Packet {
                        version: 6,
                        type_id: 4,
                        packet_type: PacketType::Literal(10),
                        subpackets: vec![],
                    },
                    Packet {
                        version: 2,
                        type_id: 4,
                        packet_type: PacketType::Literal(20),
                        subpackets: vec![],
                    },
                ],
            }
        );
    }
}
