use aoc::parsing::*;
use std::collections::*;

fn main() {
    let lines = lines();
    let start: Vec<char> = lines[0].chars().collect();
    let rules: HashMap<(char, char), char> = lines[2..]
        .iter()
        .map(|s| {
            let (outer, inner) = s.split_once(" -> ").unwrap();
            let a = outer.chars().nth(0).unwrap();
            let b = outer.chars().nth(1).unwrap();
            let inner = inner.chars().nth(0).unwrap();
            ((a, b), inner)
        })
        .collect();

    let mut state: HashMap<(char, char), usize> = HashMap::new();
    for v in start.windows(2) {
        *state.entry((v[0], v[1])).or_insert(0) += 1;
    }

    for _ in 0..40 {
        let mut next = HashMap::new();
        for (&(a, b), &count) in state.iter() {
            let inner = rules[&(a, b)];
            *next.entry((a, inner)).or_insert(0) += count;
            *next.entry((inner, b)).or_insert(0) += count;
        }
        state = next;
    }

    let mut counts = HashMap::new();
    for ((_a, b), count) in state.into_iter() {
        *counts.entry(b).or_insert(0usize) += count;
    }
    *counts.entry(start[0]).or_insert(0usize) += 1;

    println!(
        "{}",
        *counts.values().max().unwrap() - *counts.values().min().unwrap()
    );
}
