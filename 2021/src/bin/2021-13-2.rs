use std::collections::HashSet;

use aoc::parsing::*;

fn main() {
    let mut points: Vec<(u64, u64)> = Vec::new();
    let mut before = true;
    for line in lines() {
        if before {
            if line.is_empty() {
                before = false;
                continue;
            }
            let (a, b) = line.split_once(",").unwrap();
            points.push((a.parse().unwrap(), b.parse().unwrap()));
        } else {
            let line = line.strip_prefix("fold along ").unwrap();
            let (a, b) = line.split_once("=").unwrap();
            let mapper: fn(&mut (u64, u64)) -> &mut u64 = match a {
                "x" => |tup| &mut tup.0,
                "y" => |tup| &mut tup.1,
                _ => panic!("{}", a),
            };
            let b: u64 = b.parse().unwrap();
            for tup in &mut points {
                let coord = mapper(tup);
                if *coord > b {
                    *coord = 2 * b - *coord;
                }
            }
        }
    }

    let width = points.iter().map(|&(x, _)| x).max().unwrap();
    let height = points.iter().map(|&(_, y)| y).max().unwrap();
    let points: HashSet<_> = points.into_iter().collect();
    for y in 0..=height {
        for x in 0..=width {
            if points.contains(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
