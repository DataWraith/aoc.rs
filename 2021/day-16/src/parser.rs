use bitreader::BitReader;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum PacketType {
    Literal(u64),
    Operator(Vec<Packet>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Packet {
    pub version: u8,
    pub type_id: u8,
    pub contents: PacketType,
}

fn read_literal_value(input: &mut BitReader) -> u64 {
    let mut v = 0u64;

    loop {
        let segment = input.read_u8(5).unwrap();
        v = (v << 4) | (segment as u64 & 0b1111);

        if segment & 0b10000 == 0 {
            break;
        }
    }

    v
}

impl Packet {
    pub fn read(input: &mut BitReader) -> Self {
        let version = input.read_u8(3).unwrap();
        let type_id = input.read_u8(3).unwrap();

        if type_id == 4 {
            return Self {
                version,
                type_id,
                contents: PacketType::Literal(read_literal_value(input)),
            };
        }

        let length_type_id = input.read_bool().unwrap();

        if length_type_id {
            // Length is given in subpackets
            let length = input.read_u64(11).unwrap();
            let mut subpackets = vec![];

            for _ in 0..length {
                subpackets.push(Packet::read(input));
            }

            Self {
                version,
                type_id,
                contents: PacketType::Operator(subpackets),
            }
        } else {
            // Length is given in bits
            let length = input.read_u64(15).unwrap();
            let mut subpacket_data = input.relative_reader_atmost(length as u64);
            input.skip(length as u64).unwrap();

            let mut subpackets = vec![];

            while subpacket_data.remaining() > 0 {
                subpackets.push(Packet::read(&mut subpacket_data));
            }

            Self {
                version,
                type_id,
                contents: PacketType::Operator(subpackets),
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub packets: Packet,
}

pub fn part1(input: &'static str) -> PuzzleInput {
    let mut bytes = Vec::new();

    input.trim().as_bytes().chunks(2).for_each(|chunk| {
        let a = (chunk[0] as char).to_digit(16).unwrap() as u8;
        let b = (chunk[1] as char).to_digit(16).unwrap() as u8;

        bytes.push((a << 4) | b);
    });

    let mut bits = BitReader::new(&bytes);

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
                contents: PacketType::Literal(2021),
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
                contents: PacketType::Operator(vec![
                    Packet {
                        version: 6,
                        type_id: 4,
                        contents: PacketType::Literal(10),
                    },
                    Packet {
                        version: 2,
                        type_id: 4,
                        contents: PacketType::Literal(20),
                    },
                ]),
            }
        );
    }
}
