use std::{iter::FlatMap, str::Chars};

pub type HexToBits<'a> = FlatMap<Chars<'a>, [bool; 4], fn(char) -> [bool; 4]>;

pub fn hex_to_bits<'a>(s: &'a str) -> HexToBits<'a> {
    s.chars().flat_map(|c| {
        let b = c.to_digit(16).unwrap();
        [
            ((b >> 3) & 1) != 0,
            ((b >> 2) & 1) != 0,
            ((b >> 1) & 1) != 0,
            (b & 1) != 0,
        ]
    })
}

pub struct Packet {
    pub version: u64,
    pub kind: PacketKind,
}

impl Packet {
    pub fn parse(bits: &mut dyn Iterator<Item = bool>) -> Option<Self> {
        let version = decode(bits, 3)?;
        let type_id = decode(bits, 3)?;
        let kind = match type_id {
            4 => {
                let mut value = 0u64;
                loop {
                    let x = decode(bits, 5)?;
                    value <<= 4;
                    value |= x & 0xf;
                    if x & 0x10 == 0 {
                        break;
                    }
                }
                PacketKind::Literal(value)
            }
            _ => {
                let length_type = decode(bits, 1)?;
                let subpackets = if length_type == 0 {
                    let bits_length = decode(bits, 15)?;
                    let mut sub_iter = bits.take(bits_length as usize);
                    let mut subpackets = Vec::new();
                    while let Some(packet) = Packet::parse(&mut sub_iter) {
                        subpackets.push(packet);
                    }
                    subpackets
                } else {
                    let num_packets = decode(bits, 11)?;
                    (0..num_packets)
                        .map(|_| Packet::parse(bits))
                        .collect::<Option<_>>()?
                };
                let operator = match type_id {
                    0 => Operator::Sum,
                    1 => Operator::Product,
                    2 => Operator::Minimum,
                    3 => Operator::Maximum,
                    5 => Operator::Greater,
                    6 => Operator::Less,
                    7 => Operator::Equal,
                    _ => unreachable!("{}", type_id),
                };
                PacketKind::Operator {
                    subpackets,
                    operator,
                }
            }
        };
        Some(Packet { version, kind })
    }

    pub fn version_sum(&self) -> u64 {
        self.version
            + match &self.kind {
                PacketKind::Literal(_) => 0,
                PacketKind::Operator { subpackets, .. } => {
                    subpackets.iter().map(Packet::version_sum).sum::<u64>()
                }
            }
    }

    pub fn eval(&self) -> u64 {
        match &self.kind {
            PacketKind::Literal(x) => *x,
            PacketKind::Operator {
                subpackets,
                operator,
            } => match operator {
                Operator::Sum => subpackets.iter().map(Packet::eval).sum(),
                Operator::Product => subpackets.iter().map(Packet::eval).product(),
                Operator::Minimum => subpackets.iter().map(Packet::eval).min().unwrap(),
                Operator::Maximum => subpackets.iter().map(Packet::eval).max().unwrap(),
                Operator::Greater => {
                    if subpackets[0].eval() > subpackets[1].eval() {
                        1
                    } else {
                        0
                    }
                }
                Operator::Less => {
                    if subpackets[0].eval() < subpackets[1].eval() {
                        1
                    } else {
                        0
                    }
                }
                Operator::Equal => {
                    if subpackets[0].eval() == subpackets[1].eval() {
                        1
                    } else {
                        0
                    }
                }
            },
        }
    }
}

pub enum PacketKind {
    Literal(u64),
    Operator {
        subpackets: Vec<Packet>,
        operator: Operator,
    },
}

pub enum Operator {
    Sum,
    Product,
    Minimum,
    Maximum,
    Greater,
    Less,
    Equal,
}

fn decode(bits: &mut dyn Iterator<Item = bool>, n: usize) -> Option<u64> {
    let mut acc = 0;
    for _ in 0..n {
        acc <<= 1;
        acc |= bits.next()? as u64;
    }
    Some(acc)
}
