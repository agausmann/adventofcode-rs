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

    let mut state = start;

    for _ in 0..10 {
        state = [state[0]]
            .into_iter()
            .chain(state.windows(2).flat_map(|v| [rules[&(v[0], v[1])], v[1]]))
            .collect();
    }

    let mut counts = HashMap::new();
    for c in state {
        *counts.entry(c).or_insert(0usize) += 1;
    }
    println!(
        "{}",
        *counts.values().max().unwrap() - *counts.values().min().unwrap()
    );
}
