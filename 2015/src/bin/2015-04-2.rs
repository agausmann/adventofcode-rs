use std::{fmt::Write as _, io::Write as _};

use aoc::parsing::lines;
use md5::{Digest, Md5};

fn main() {
    for secret in lines() {
        let mut buf = Vec::new();
        let mut hex = String::new();
        for i in 0.. {
            buf.clear();
            hex.clear();

            write!(buf, "{}{}", secret, i).unwrap();

            let mut hasher = Md5::new();
            hasher.update(&buf);
            let digest: [u8; 16] = hasher.finalize().into();

            for b in digest {
                write!(hex, "{:02x}", b).unwrap();
            }

            if hex.starts_with("000000") {
                println!("{}", i);
                break;
            }
        }
    }
}
