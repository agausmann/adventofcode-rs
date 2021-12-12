use std::collections::{hash_set, HashMap, HashSet};

use aoc::parsing::lines;

fn main() {
    let lines = lines();
    let mut edges: HashMap<&str, HashSet<&str>> = HashMap::new();
    for line in &lines {
        let (a, b) = line.split_once("-").unwrap();
        edges.entry(a).or_default().insert(b);
        edges.entry(b).or_default().insert(a);
    }

    let mut caves = HashSet::new();
    let mut stack = Vec::new();
    caves.insert("start");
    stack.push(Frame {
        current: "start",
        iter: edges["start"].iter(),
    });
    let mut paths = 0;

    while let Some(head) = stack.last_mut() {
        if let Some(&next) = head.iter.next() {
            if next == "end" {
                paths += 1;
            } else if next.chars().all(|c| c.is_ascii_uppercase()) || !caves.contains(next) {
                caves.insert(next);
                stack.push(Frame {
                    current: next,
                    iter: edges[next].iter(),
                })
            }
        } else {
            caves.remove(head.current);
            stack.pop();
        }
    }

    println!("{}", paths);
}

pub struct Frame<'a> {
    current: &'a str,
    iter: hash_set::Iter<'a, &'a str>,
}
