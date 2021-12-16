use core::panic;

fn main() {
    let input = "4054460802532B12FEE8B180213B19FA5AA77601C010E4EC2571A9EDFE356C7008E7B141898C1F4E50DA7438C011D005E4F6E727B738FC40180CB3ED802323A8C3FED8C4E8844297D88C578C26008E004373BCA6B1C1C99945423798025800D0CFF7DC199C9094E35980253FB50A00D4C401B87104A0C8002171CE31C41201062C01393AE2F5BCF7B6E969F3C553F2F0A10091F2D719C00CD0401A8FB1C6340803308A0947B30056803361006615C468E4200E47E8411D26697FC3F91740094E164DFA0453F46899015002A6E39F3B9802B800D04A24CC763EDBB4AFF923A96ED4BDC01F87329FA491E08180253A4DE0084C5B7F5B978CC410012F9CFA84C93900A5135BD739835F00540010F8BF1D22A0803706E0A47B3009A587E7D5E4D3A59B4C00E9567300AE791E0DCA3C4A32CDBDC4830056639D57C00D4C401C8791162380021108E26C6D991D10082549218CDC671479A97233D43993D70056663FAC630CB44D2E380592FB93C4F40CA7D1A60FE64348039CE0069E5F565697D59424B92AF246AC065DB01812805AD901552004FDB801E200738016403CC000DD2E0053801E600700091A801ED20065E60071801A800AEB00151316450014388010B86105E13980350423F447200436164688A4001E0488AC90FCDF31074929452E7612B151803A200EC398670E8401B82D04E31880390463446520040A44AA71C25653B6F2FE80124C9FF18EDFCA109275A140289CDF7B3AEEB0C954F4B5FC7CD2623E859726FB6E57DA499EA77B6B68E0401D996D9C4292A881803926FB26232A133598A118023400FA4ADADD5A97CEEC0D37696FC0E6009D002A937B459BDA3CC7FFD65200F2E531581AD80230326E11F52DFAEAAA11DCC01091D8BE0039B296AB9CE5B576130053001529BE38CDF1D22C100509298B9950020B309B3098C002F419100226DC";

    let bits = to_binary(input);

    println!("p1: {}", p1(&bits));
    // 889

    println!("p2: {}", p2(&bits));
    //739303923668
}

fn p1(bits: &Vec<u8>) -> u64 {
    let packet = parse(bits);
    packet.version_sum()
}

fn p2(bits: &Vec<u8>) -> u64 {
    let packet = parse(bits);
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
                    _ => todo!(),
                }
            }
        }
    }
}

fn parse(bits: &Vec<u8>) -> Packet {
    let (packet, _) = parse_packet(bits, 0);
    packet
}

fn parse_packet(bits: &[u8], offset: usize) -> (Packet, usize) {
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
    let mut litvalue: Vec<u8> = vec![];

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
    let mut children: Vec<Packet> = vec![];

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
    let mut children: Vec<Packet> = vec![];

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

fn to_decimal(bin_slize: &[u8]) -> u64 {
    let (res, _) = bin_slize.iter().rev().fold((0, 0), |(res, power), bit| {
        (res + ((*bit as u64) << power), power + 1)
    });

    res
}

fn to_binary(hexstr: &str) -> Vec<u8> {
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

#[cfg(test)]
mod tests {
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
            parse(&to_binary("D2FE28")),
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
            parse(&to_binary("38006F45291200")),
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
            parse(&to_binary("EE00D40C823060")),
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
    }
}
