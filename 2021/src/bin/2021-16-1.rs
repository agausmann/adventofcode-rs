use aoc::parsing::*;

fn main() {
    let bits: Vec<u8> = lines()[0]
        .chars()
        .flat_map(|c| {
            let b = c.to_digit(16).unwrap() as u8;
            [(b >> 3) & 1, (b >> 2) & 1, (b >> 1) & 1, b & 1]
        })
        .collect();

    let mut iter = bits.iter().copied();
    let packet = Packet::parse(&mut iter).unwrap();
    println!("{}", packet.version_sum());
}

struct Packet {
    version: u64,
    kind: PacketKind,
}

impl Packet {
    fn parse(bits: &mut dyn Iterator<Item = u8>) -> Option<Self> {
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
                PacketKind::Other { subpackets }
            }
        };
        Some(Packet { version, kind })
    }

    fn version_sum(&self) -> u64 {
        self.version
            + match &self.kind {
                PacketKind::Literal(_) => 0,
                PacketKind::Other { subpackets } => {
                    subpackets.iter().map(Packet::version_sum).sum::<u64>()
                }
            }
    }
}

enum PacketKind {
    Literal(u64),
    Other { subpackets: Vec<Packet> },
}

fn decode(bits: &mut dyn Iterator<Item = u8>, n: usize) -> Option<u64> {
    let mut acc = 0;
    for _ in 0..n {
        acc <<= 1;
        acc |= bits.next()? as u64;
    }
    Some(acc)
}
