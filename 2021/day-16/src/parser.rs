use utility_belt::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub enum PacketType {
    Literal(u64),
    OperatorLength(u64),
    OperatorSubpackets(u64),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Packet {
    pub version: u8,
    pub type_id: u8,
    pub value: PacketType,
    pub subpackets: Vec<Packet>,
}

fn read_bits(input: &mut VecDeque<char>, n: usize) -> u64 {
    let mut ans = 0;

    for _ in 0..n {
        ans = (ans << 1) | (input.pop_front().unwrap() == '1') as u64;
    }

    ans
}

fn read_literal_value(input: &mut VecDeque<char>, ans: &mut Vec<u8>) {
    let b1 = (input.pop_front().unwrap() == '1') as u8;
    let b2 = (input.pop_front().unwrap() == '1') as u8;
    let b3 = (input.pop_front().unwrap() == '1') as u8;
    let b4 = (input.pop_front().unwrap() == '1') as u8;
    let b5 = (input.pop_front().unwrap() == '1') as u8;

    ans.push((b2 << 3) | (b3 << 2) | (b4 << 1) | b5);

    if b1 == 1 {
        read_literal_value(input, ans);
    }
}

fn read_operator_packet_length(input: &mut VecDeque<char>) -> PacketType {
    let length_type_id = input.pop_front().unwrap();

    if length_type_id == '0' {
        let length = read_bits(input, 15);
        PacketType::OperatorLength(length)
    } else {
        let length = read_bits(input, 11);
        PacketType::OperatorSubpackets(length)
    }
}

fn convert_literal_value(input: &[u8]) -> PacketType {
    let mut value = 0u64;

    assert!(input.len() <= 64 / 4);

    for b in input.iter() {
        value = (value << 4) | *b as u64;
    }

    PacketType::Literal(value)
}

impl Packet {
    pub fn read(input: &mut VecDeque<char>) -> Self {
        let b1 = (input.pop_front().unwrap() == '1') as u8;
        let b2 = (input.pop_front().unwrap() == '1') as u8;
        let b3 = (input.pop_front().unwrap() == '1') as u8;

        let b4 = (input.pop_front().unwrap() == '1') as u8;
        let b5 = (input.pop_front().unwrap() == '1') as u8;
        let b6 = (input.pop_front().unwrap() == '1') as u8;

        let version = (b1 << 2) | (b2 << 1) | b3;
        let type_id = (b4 << 2) | (b5 << 1) | b6;

        if type_id == 4 {
            let mut ans = vec![];
            read_literal_value(input, &mut ans);

            return Self {
                version,
                type_id,
                value: convert_literal_value(&ans),
                subpackets: vec![],
            };
        }

        match read_operator_packet_length(input) {
            PacketType::Literal(_) => unreachable!("We already checked for literals"),

            PacketType::OperatorLength(length) => {
                let remainder = input.split_off(length as usize);

                let mut subpackets = vec![];

                while !input.is_empty() {
                    subpackets.push(Packet::read(input));
                }

                *input = remainder;

                Self {
                    version,
                    type_id,
                    value: PacketType::OperatorLength(length),
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
                    value: PacketType::OperatorSubpackets(length),
                    subpackets,
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub packets: VecDeque<char>,
}

pub fn part1(input: &'static str) -> PuzzleInput {
    let mut packets = VecDeque::new();

    input.chars().filter_map(|c| c.to_digit(16)).for_each(|d| {
        for i in (0..4).rev() {
            if d & (1 << i) != 0 {
                packets.push_back('1');
            } else {
                packets.push_back('0');
            }
        }
    });

    PuzzleInput { packets }
}

pub fn part2(input: &'static str) -> PuzzleInput {
    part1(input)
}

pub const TEST_INPUT1: &str = indoc! {"
    D2FE28
"};

pub const TEST_INPUT2: &str = indoc! {"
    38006F45291200
"};

pub const TEST_INPUT3: &str = indoc! {"
    A0016C880162017C3686B18A3D4780
"};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_example1() {
        let input = part1(TEST_INPUT1);
        let packet = Packet::read(&mut input.packets.clone());
        assert_eq!(
            packet,
            Packet {
                version: 6,
                type_id: 4,
                value: PacketType::Literal(2021),
                subpackets: vec![],
            }
        );
    }

    #[test]
    fn test_parse_example2() {
        let input = part1(TEST_INPUT2);
        let packet = Packet::read(&mut input.packets.clone());
        assert_eq!(
            packet,
            Packet {
                version: 1,
                type_id: 6,
                value: PacketType::OperatorLength(27),
                subpackets: vec![
                    Packet {
                        version: 6,
                        type_id: 4,
                        value: PacketType::Literal(10),
                        subpackets: vec![],
                    },
                    Packet {
                        version: 2,
                        type_id: 4,
                        value: PacketType::Literal(20),
                        subpackets: vec![],
                    },
                ],
            }
        );
    }
}
