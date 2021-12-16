use aoc::parsing::*;
use aoc2021::packet::{hex_to_bits, Packet};

fn main() {
    let packet = Packet::parse(&mut hex_to_bits(&lines()[0])).unwrap();
    println!("{}", packet.eval());
}
