use core::panic;
use std::fs;

fn main() {
    let input = fs::read_to_string("d16_input").unwrap();
    let bits = converters::to_binary(&input.trim_end());

    println!("p1 = {}", p1(&bits));
    println!("p2 = {}", p2(&bits));
}

fn p1(bits: &Vec<u8>) -> u64 {
    let packet = Packet::parse(bits);
    packet.version_sum()
}

fn p2(bits: &Vec<u8>) -> u64 {
    let packet = Packet::parse(bits);
    packet.eval()
}

#[derive(Debug, PartialEq, Eq)]
pub enum Packet {
    Literal {
        version: u64,
        type_id: u64,
        value: u64,
    },
    Operator {
        version: u64,
        type_id: u64,
        children: Vec<Packet>,
    },
}

impl Packet {
    fn version_sum(&self) -> u64 {
        match self {
            Packet::Literal { version, .. } => *version,
            Packet::Operator {
                version, children, ..
            } => version + children.iter().map(|c| c.version_sum()).sum::<u64>(),
        }
    }

    fn eval(&self) -> u64 {
        match self {
            Packet::Literal { value, .. } => *value,
            Packet::Operator {
                type_id, children, ..
            } => {
                let mut vs = children.iter().map(|c| c.eval()); //.collect::<Vec<_>>();

                match type_id {
                    0 => vs.sum(),
                    1 => vs.product(),
                    2 => vs.min().unwrap(),
                    3 => vs.max().unwrap(),
                    5 => {
                        let first = vs.next();
                        let second = vs.next();
                        if first > second {
                            1
                        } else {
                            0
                        }
                    }
                    6 => {
                        let first = vs.next();
                        let second = vs.next();
                        if first < second {
                            1
                        } else {
                            0
                        }
                    }
                    7 => {
                        let first = vs.next();
                        let second = vs.next();
                        if first == second {
                            1
                        } else {
                            0
                        }
                    }
                    _ => panic!("Unknown operator with type_id = {}", type_id),
                }
            }
        }
    }
}

impl Packet {
    fn parse(bits: &[u8]) -> Packet {
        let (packet, _) = parsers::parse_packet(bits, 0);
        packet
    }
}

mod parsers {
    use super::*;
    use converters::*;
    use core::panic;

    pub fn parse_packet(bits: &[u8], offset: usize) -> (Packet, usize) {
        let mut new_offset = offset;

        let version = to_decimal(&bits[new_offset..new_offset + 3]);
        new_offset += 3;

        let type_id = to_decimal(&bits[new_offset..new_offset + 3]);
        new_offset += 3;

        if type_id == 4 {
            parse_literal(bits, version, type_id, new_offset)
        } else {
            parse_operator(bits, version, type_id, new_offset)
        }
    }

    fn parse_literal(bits: &[u8], version: u64, type_id: u64, offset: usize) -> (Packet, usize) {
        let mut new_offset = offset;
        let mut litvalue = vec![];

        loop {
            let is_terminal = bits[new_offset];
            new_offset += 1;

            litvalue.extend(&bits[new_offset..new_offset + 4]);
            new_offset += 4;

            if is_terminal == 0 {
                let value = to_decimal(&litvalue);

                return (
                    Packet::Literal {
                        version,
                        type_id,
                        value,
                    },
                    new_offset,
                );
            }
        }
    }

    fn parse_operator(bits: &[u8], version: u64, type_id: u64, offset: usize) -> (Packet, usize) {
        let mut new_offset = offset;

        let length_type_id = bits[new_offset];
        new_offset += 1;

        if length_type_id == 0 {
            let total_length = to_decimal(&bits[new_offset..new_offset + 15]) as usize;
            new_offset += 15;

            parse_operator_with_total_length(bits, version, type_id, total_length, new_offset)
        } else if length_type_id == 1 {
            let children_count = to_decimal(&bits[new_offset..new_offset + 11]);
            new_offset += 11;

            parse_operator_with_children_count(bits, version, type_id, children_count, new_offset)
        } else {
            panic!("unexpected length type id = {}", length_type_id);
        }
    }

    fn parse_operator_with_total_length(
        bits: &[u8],
        version: u64,
        type_id: u64,
        total_length: usize,
        offset: usize,
    ) -> (Packet, usize) {
        let mut new_offset = offset;
        let mut children = vec![];

        while new_offset < offset + total_length {
            let (child, child_offset) = parse_packet(bits, new_offset);

            children.push(child);
            new_offset = child_offset;
        }

        if new_offset != offset + total_length {
            panic!(
                "Offset ({}) + total length ({}) is not equal to the offset after reading children: {}.",
                offset, total_length, new_offset
            )
        }

        (
            Packet::Operator {
                version,
                type_id,
                children,
            },
            new_offset,
        )
    }

    fn parse_operator_with_children_count(
        bits: &[u8],
        version: u64,
        type_id: u64,
        children_count: u64,
        offset: usize,
    ) -> (Packet, usize) {
        let mut new_offset = offset;
        let mut children = vec![];

        for _ in 0..children_count {
            let (child, child_offset) = parse_packet(bits, new_offset);

            children.push(child);
            new_offset = child_offset;
        }

        (
            Packet::Operator {
                version,
                type_id,
                children,
            },
            new_offset,
        )
    }
}

mod converters {
    pub fn to_decimal(bin_slize: &[u8]) -> u64 {
        let (res, _) = bin_slize.iter().rev().fold((0, 0), |(res, power), bit| {
            (res + ((*bit as u64) << power), power + 1)
        });

        res
    }

    pub fn to_binary(hexstr: &str) -> Vec<u8> {
        let mut res = Vec::with_capacity(hexstr.len() * 4);

        for ch in hexstr.chars() {
            match ch {
                '0' => res.extend([0, 0, 0, 0]),
                '1' => res.extend([0, 0, 0, 1]),
                '2' => res.extend([0, 0, 1, 0]),
                '3' => res.extend([0, 0, 1, 1]),
                '4' => res.extend([0, 1, 0, 0]),
                '5' => res.extend([0, 1, 0, 1]),
                '6' => res.extend([0, 1, 1, 0]),
                '7' => res.extend([0, 1, 1, 1]),
                '8' => res.extend([1, 0, 0, 0]),
                '9' => res.extend([1, 0, 0, 1]),
                'A' => res.extend([1, 0, 1, 0]),
                'B' => res.extend([1, 0, 1, 1]),
                'C' => res.extend([1, 1, 0, 0]),
                'D' => res.extend([1, 1, 0, 1]),
                'E' => res.extend([1, 1, 1, 0]),
                'F' => res.extend([1, 1, 1, 1]),
                _ => todo!(),
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::converters::*;
    use super::*;
    use Packet::*;

    #[test]
    fn to_binary_test() {
        assert_eq!(
            to_binary("D2FE28"),
            [1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0]
        );
    }

    #[test]
    fn to_decimal_test() {
        assert_eq!(to_decimal(&[1, 0, 0]), 4);
        assert_eq!(to_decimal(&[1, 1, 0, 1, 1]), 27);
        assert_eq!(to_decimal(&[0, 0, 0, 1, 1, 0, 1, 1]), 27);
        assert_eq!(to_decimal(&[0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 1, 0, 1]), 2021);
    }

    #[test]
    fn parse_literal_test() {
        assert_eq!(
            Packet::parse(&to_binary("D2FE28")),
            Packet::Literal {
                version: 6,
                type_id: 4,
                value: 2021
            }
        )
    }

    #[test]
    fn parse_operator_test() {
        assert_eq!(
            Packet::parse(&to_binary("38006F45291200")),
            Packet::Operator {
                version: 1,
                type_id: 6,
                children: vec![
                    Packet::Literal {
                        version: 6,
                        type_id: 4,
                        value: 10
                    },
                    Packet::Literal {
                        version: 2,
                        type_id: 4,
                        value: 20
                    }
                ]
            }
        );

        assert_eq!(
            Packet::parse(&to_binary("EE00D40C823060")),
            Operator {
                version: 7,
                type_id: 3,
                children: vec![
                    Literal {
                        version: 2,
                        type_id: 4,
                        value: 1
                    },
                    Literal {
                        version: 4,
                        type_id: 4,
                        value: 2
                    },
                    Literal {
                        version: 1,
                        type_id: 4,
                        value: 3
                    }
                ]
            }
        );
    }

    #[test]
    fn p1_test() {
        assert_eq!(p1(&to_binary("A0016C880162017C3686B18A3D4780")), 31);

        let input = fs::read_to_string("../d16_input").unwrap();
        assert_eq!(p1(&to_binary(&input.trim_end())), 889);
    }

    #[test]
    fn p2_test() {
        assert_eq!(p2(&to_binary("C200B40A82")), 3);
        assert_eq!(p2(&to_binary("04005AC33890")), 54);
        assert_eq!(p2(&to_binary("880086C3E88112")), 7);
        assert_eq!(p2(&to_binary("CE00C43D881120")), 9);
        assert_eq!(p2(&to_binary("D8005AC2A8F0")), 1);
        assert_eq!(p2(&to_binary("F600BC2D8F")), 0);
        assert_eq!(p2(&to_binary("9C005AC2F8F0")), 0);
        assert_eq!(p2(&to_binary("9C0141080250320F1802104A08")), 1);

        let input = fs::read_to_string("../d16_input").unwrap();
        assert_eq!(p2(&to_binary(&input.trim_end())), 739303923668);
    }
}
