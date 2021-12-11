use std::collections::HashSet;

use aoc::{parsing::lines, point2d::Point2d};

fn main() {
    for line in lines() {
        let mut p: Point2d<i32> = Point2d::zero();
        let mut v = HashSet::new();
        v.insert(p);

        for c in line.chars() {
            match c {
                '^' => p = p.up(),
                'v' => p = p.down(),
                '<' => p = p.left(),
                '>' => p = p.right(),
                _ => panic!("{}", c),
            }
            v.insert(p);
        }
        println!("{}", v.len());
    }
}
