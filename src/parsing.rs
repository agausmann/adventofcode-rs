use std::io::{stdin, Read};

pub fn input() -> String {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    input
}

pub fn lines() -> Vec<String> {
    input().lines().map(Into::into).collect()
}
