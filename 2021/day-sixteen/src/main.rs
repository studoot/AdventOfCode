use bitvec::mem::BitMemory;
use bitvec::prelude::*;
use hex::FromHex;
use std::path::{Component, Path};

#[derive(Debug, PartialEq, Eq)]

enum Op {
    Sum,
    Product,
    Maximum,
    Minimum,
    Greater,
    Less,
    Equal,
}

#[derive(Debug, PartialEq, Eq)]
enum Payload {
    Literal(u64),
    Operator(Op, Vec<Packet>),
}
#[derive(Debug, PartialEq, Eq)]
struct Packet {
    version: u8,
    type_id: u8,
    payload: Payload,
}

impl Packet {
    fn version_sum(&self) -> usize {
        match &self.payload {
            Payload::Literal(_) => self.version as usize,
            Payload::Operator(_, sub_packets) => {
                (self.version as usize) + sub_packets.iter().map(|p| p.version_sum()).sum::<usize>()
            }
        }
    }
    fn calculate(&self) -> u64 {
        match &self.payload {
            Payload::Literal(value) => *value,
            Payload::Operator(op_type, sub_packets) => match op_type {
                Op::Sum => sub_packets.iter().map(Packet::calculate).sum(),
                Op::Product => sub_packets.iter().map(Packet::calculate).product(),
                Op::Maximum => sub_packets.iter().map(Packet::calculate).max().unwrap(),
                Op::Minimum => sub_packets.iter().map(Packet::calculate).min().unwrap(),
                Op::Greater => (sub_packets[0].calculate() > sub_packets[1].calculate()) as u64,
                Op::Less => (sub_packets[0].calculate() < sub_packets[1].calculate()) as u64,
                Op::Equal => (sub_packets[0].calculate() == sub_packets[1].calculate()) as u64,
            },
        }
    }
}

type Bits = BitSlice<Msb0, u8>;

fn consume_field<T: BitMemory>(bits: &Bits, field_size: usize) -> Option<(T, &Bits)> {
    if bits.len() < field_size {
        None
    } else {
        let (field_bits, rest) = bits.split_at(field_size);
        Some((field_bits.load_be::<T>(), rest))
    }
}

fn consume_bool(bits: &Bits) -> Option<(bool, &Bits)> {
    let (a, bits) = consume_field::<u8>(bits, 1)?;
    Some((a == 1, bits))
}

fn parse_packet(bits: &Bits) -> Option<(Packet, &Bits)> {
    if bits.len() < 6 || bits.leading_ones() == bits.leading_zeros() || bits.len() == bits.count_zeros() {
        return None;
    }
    let (version, bits) = consume_field::<u8>(bits, 3)?;
    let (type_id, mut bits) = consume_field::<u8>(bits, 3)?;
    let payload = match type_id {
        4 => {
            let mut acc = 0;
            loop {
                let (prefix, rest) = consume_bool(bits)?;
                let (nibble, rest) = consume_field::<u64>(rest, 4)?;
                acc = (acc << 4) + nibble;
                bits = rest;
                if !prefix {
                    break;
                }
            }
            Payload::Literal(acc)
        }
        op_id => {
            let operator = match op_id {
                0 => Op::Sum,
                1 => Op::Product,
                2 => Op::Minimum,
                3 => Op::Maximum,
                5 => Op::Greater,
                6 => Op::Less,
                7 => Op::Equal,
                _ => unreachable!(),
            };
            let mut sub_packets = Vec::new();
            let (length_type_id, rest) = consume_bool(bits)?;
            if length_type_id {
                let (sub_packet_count, mut sub_packet_bits) = consume_field::<u16>(rest, 11)?;
                for _ in 0..sub_packet_count {
                    if let Some((p, rest)) = parse_packet(sub_packet_bits) {
                        sub_packets.push(p);
                        sub_packet_bits = rest;
                    }
                }
                bits = sub_packet_bits;
            } else {
                let (sub_packet_bit_length, sub_packet_bits) = consume_field::<usize>(rest, 15)?;
                let (mut sub_packet_bits, rest) = sub_packet_bits.split_at(sub_packet_bit_length);
                bits = rest;
                while let Some((p, rest)) = parse_packet(sub_packet_bits) {
                    sub_packets.push(p);
                    sub_packet_bits = rest;
                }
            }
            Payload::Operator(operator, sub_packets)
        }
    };
    Some((Packet { version, type_id, payload }, bits))
}

fn parse_packets(bytes: &[u8]) -> Vec<Packet> {
    let mut bits = bytes.view_bits::<Msb0>();
    let mut packets = Vec::new();
    while let Some((p, rest)) = parse_packet(bits) {
        packets.push(p);
        bits = rest;
    }
    packets
}
mod part1 {
    use super::*;

    pub fn run(input: &str) -> usize {
        let bytes = <Vec<u8>>::from_hex(input.lines().next().unwrap()).unwrap();
        parse_packets(&bytes)
            .into_iter()
            .map(|p| p.version_sum() as usize)
            .sum()
    }

    #[test]
    fn test_run() {
        assert_eq!(run("D2FE28"), 6);
        assert_eq!(run("38006F45291200"), 9);
        assert_eq!(run("EE00D40C823060"), 14);
        assert_eq!(run("8A004A801A8002F478"), 16);
        assert_eq!(run("8A004A801A8002F478"), 16);
        assert_eq!(run("620080001611562C8802118E34"), 12);
        assert_eq!(run("C0015000016115A2E0802F182340"), 23);
        assert_eq!(run("A0016C880162017C3686B18A3D4780"), 31);
    }
}

mod part2 {
    use super::*;

    pub fn run(input: &str) -> u64 {
        let bytes = <Vec<u8>>::from_hex(input.lines().next().unwrap()).unwrap();
        let packets = parse_packets(&bytes);
        assert_eq!(packets.len(), 1);
        packets[0].calculate()
    }

    #[test]
    fn test_run() {
        assert_eq!(run("C200B40A82"), 3);
        assert_eq!(run("04005AC33890"), 54);
        assert_eq!(run("880086C3E88112"), 7);
        assert_eq!(run("CE00C43D881120"), 9);
        assert_eq!(run("D8005AC2A8F0"), 1);
        assert_eq!(run("F600BC2D8F"), 0);
        assert_eq!(run("9C005AC2F8F0"), 0);
        assert_eq!(run("9C0141080250320F1802104A08"), 1);
    }
}

fn main() {
    let input_string = include_str!("../input.txt");
    let day_number = Path::new(file!())
        .components()
        .find_map(|bit| {
            if let Component::Normal(os_name) = bit {
                if let Some(dir_name) = os_name.to_str() {
                    return dir_name.strip_prefix("day-");
                }
            };
            None
        })
        .unwrap()
        .to_lowercase()
        .replace("-", " ")
        .replace("_", " ");
    let now = std::time::Instant::now();
    let part1_ans = part1::run(input_string);
    println!("Day {} part 1 - {} - took {} milliseconds.", day_number, part1_ans, now.elapsed().as_millis());
    assert_eq!(part1_ans, 908);

    let now = std::time::Instant::now();
    let part2_ans = part2::run(input_string);
    println!("Day {} part 2 - {} - took {} milliseconds.", day_number, part2_ans, now.elapsed().as_millis());
    assert_eq!(part2_ans, 10626195124371);
}
