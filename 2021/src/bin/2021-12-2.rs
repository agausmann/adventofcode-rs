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
    let mut second_cave = None;
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
            } else if next.chars().all(|c| c.is_ascii_uppercase())
                || !caves.contains(next)
                || (second_cave.is_none() && next != "start")
            {
                stack.push(Frame {
                    current: next,
                    iter: edges[next].iter(),
                });
                if next.chars().all(|c| c.is_ascii_lowercase()) && caves.contains(next) {
                    assert!(second_cave.is_none());
                    second_cave = Some(next);
                }
                caves.insert(next);
            }
        } else {
            if second_cave == Some(head.current) {
                second_cave = None;
            } else {
                caves.remove(head.current);
            }
            stack.pop();
        }
    }

    println!("{}", paths);
}

pub struct Frame<'a> {
    current: &'a str,
    iter: hash_set::Iter<'a, &'a str>,
}
