use std::collections::HashSet;

use aoc::{parsing::lines, point2d::Point2d};

fn main() {
    for line in lines() {
        let mut v = HashSet::new();

        let mut p: Point2d<i32> = Point2d::zero();
        v.insert(p);
        for c in line.chars().step_by(2) {
            match c {
                '^' => p = p.up(),
                'v' => p = p.down(),
                '<' => p = p.left(),
                '>' => p = p.right(),
                _ => panic!("{}", c),
            }
            v.insert(p);
        }

        let mut p: Point2d<i32> = Point2d::zero();
        v.insert(p);
        for c in line.chars().skip(1).step_by(2) {
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
