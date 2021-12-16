use std::fs;

mod converters;
mod parsers;

fn main() {
    let input = fs::read_to_string("d16_input").unwrap();
    let packet: Packet = input.trim_end().into();

    println!("p1 = {}", packet.version_sum());
    println!("p2 = {}", packet.eval());
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
    pub fn version_sum(&self) -> u64 {
        match self {
            Packet::Literal { version, .. } => *version,
            Packet::Operator {
                version, children, ..
            } => version + children.iter().map(|c| c.version_sum()).sum::<u64>(),
        }
    }

    pub fn eval(&self) -> u64 {
        match self {
            Packet::Literal { value, .. } => *value,
            Packet::Operator {
                type_id, children, ..
            } => {
                let mut vs = children.iter().map(|c| c.eval());

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

impl From<&Vec<u8>> for Packet {
    fn from(bits: &Vec<u8>) -> Self {
        bits.into()
    }
}

impl From<Vec<u8>> for Packet {
    fn from(bits: Vec<u8>) -> Self {
        (&bits[..]).into()
    }
}

impl From<&[u8]> for Packet {
    fn from(bits: &[u8]) -> Self {
        let (packet, _) = parsers::parse_packet(bits, 0);
        packet
    }
}

impl From<&str> for Packet {
    fn from(hexstr: &str) -> Self {
        converters::to_binary(hexstr).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Packet::*;

    #[test]
    fn parse_literal_test() {
        assert_eq!(
            Packet::Literal {
                version: 6,
                type_id: 4,
                value: 2021
            },
            "D2FE28".into()
        )
    }

    #[test]
    fn parse_operator_test() {
        assert_eq!(
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
            },
            "38006F45291200".into()
        );

        assert_eq!(
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
            },
            "EE00D40C823060".into()
        );
    }

    #[test]
    fn p1_test() {
        let packet: Packet = "A0016C880162017C3686B18A3D4780".into();
        assert_eq!(packet.version_sum(), 31);

        let input = fs::read_to_string("../d16_input").unwrap();
        let packet: Packet = input.trim_end().into();
        assert_eq!(packet.version_sum(), 889);
    }

    #[test]
    fn p2_test() {
        assert_eq!(Packet::from("C200B40A82").eval(), 3);
        assert_eq!(Packet::from("04005AC33890").eval(), 54);
        assert_eq!(Packet::from("880086C3E88112").eval(), 7);
        assert_eq!(Packet::from("CE00C43D881120").eval(), 9);
        assert_eq!(Packet::from("D8005AC2A8F0").eval(), 1);
        assert_eq!(Packet::from("F600BC2D8F").eval(), 0);
        assert_eq!(Packet::from("9C005AC2F8F0").eval(), 0);
        assert_eq!(Packet::from("9C0141080250320F1802104A08").eval(), 1);

        let input = fs::read_to_string("../d16_input").unwrap();
        assert_eq!(Packet::from(input.trim_end()).eval(), 739303923668);
    }
}
