use core::panic;

use super::*;
use converters::*;

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
