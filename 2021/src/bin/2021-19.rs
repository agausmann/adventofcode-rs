use std::collections::{HashMap, HashSet};

use aoc::parsing::input;
use nalgebra::{matrix, vector, Matrix3, Vector3};
use once_cell::sync::Lazy;

fn main() {
    let input = input();
    let scanners: Vec<Scanner> = input
        .split("\n\n")
        .map(|s| {
            let beacons = s
                .lines()
                .skip(1)
                .map(|line| {
                    Vector3::from_iterator(line.split(',').map(|s| s.parse::<i64>().unwrap()))
                })
                .collect();
            Scanner { beacons }
        })
        .collect();

    let mut open_set: HashSet<usize> = HashSet::new();
    let mut positions: HashMap<usize, Vector3<i64>> = HashMap::new();
    let mut rotations: HashMap<usize, Matrix3<i64>> = HashMap::new();
    let mut beacons: HashSet<Vector3<i64>> = HashSet::new();
    open_set.insert(0);
    positions.insert(0, vector![0, 0, 0]);
    rotations.insert(
        0,
        matrix![
            1, 0, 0;
            0, 1, 0;
            0, 0, 1;
        ],
    );
    for &b in &scanners[0].beacons {
        beacons.insert(b);
    }

    while let Some(&i) = open_set.iter().next() {
        open_set.remove(&i);
        for j in 0..scanners.len() {
            eprintln!("{} {}", i, j);
            if positions.contains_key(&j) {
                continue;
            }
            if let Some(corr) = scanners[i].correlation(&scanners[j]) {
                let scanner_position = positions[&i] + rotations[&i] * corr.offset;
                let scanner_rotation = rotations[&i] * corr.rotation;
                for &b in &scanners[j].beacons {
                    beacons.insert(scanner_position + scanner_rotation * b);
                }
                open_set.insert(j);
                positions.insert(j, scanner_position);
                rotations.insert(j, scanner_rotation);
            }
        }
    }
    println!("{}", beacons.len());
    let max_distance = positions
        .values()
        .flat_map(|a| positions.values().map(move |b| (a - b).abs().sum()))
        .max()
        .unwrap();
    println!("{}", max_distance);

    // let mut correlations = Vec::new();
    // for i in 0..scanners.len() {
    //     for j in i + 1..scanners.len() {
    //         if let Some(corr) = scanners[i].correlation(&scanners[j]) {
    //             eprintln!("{} {} {:?}", i, j, corr);
    //             correlations.push(corr);
    //         }
    //     }
    // }

    // let detected: usize = scanners.iter().map(|s| s.beacons.len()).sum();
    // let doubles: usize = correlations.iter().map(|c| c.num_matches).sum();
    // println!("{}", detected - doubles);
}

struct Scanner {
    beacons: Vec<Vector3<i64>>,
}

impl Scanner {
    fn correlation(&self, other: &Scanner) -> Option<Correlation> {
        for rotation in ROTATIONS.iter() {
            for i in 0..self.beacons.len() {
                for j in 0..other.beacons.len() {
                    let my_base = self.beacons[i];
                    let other_base = rotation * other.beacons[j];
                    let my_offsets: HashSet<Vector3<i64>> =
                        self.beacons.iter().copied().map(|v| v - my_base).collect();
                    let other_offsets: HashSet<Vector3<i64>> = other
                        .beacons
                        .iter()
                        .copied()
                        .map(|v| (rotation * v) - other_base)
                        .collect();
                    let num_matches = my_offsets.intersection(&other_offsets).count();
                    if num_matches >= 12 {
                        return Some(Correlation {
                            offset: my_base - other_base,
                            rotation,
                        });
                    }
                }
            }
        }
        None
    }
}

#[derive(Debug)]
struct Correlation {
    offset: Vector3<i64>,
    rotation: &'static Matrix3<i64>,
}

fn rotations() -> impl Iterator<Item = Matrix3<i64>> {
    REFLECTIONS
        .iter()
        .flat_map(|r| SWIZZLES.iter().map(|s| Matrix3::from_diagonal(r) * s))
}

static ROTATIONS: Lazy<Vec<Matrix3<i64>>> = Lazy::new(|| rotations().collect());

const REFLECTIONS: [Vector3<i64>; 8] = [
    vector![1, 1, 1],
    vector![1, 1, -1],
    vector![1, -1, 1],
    vector![1, -1, -1],
    vector![-1, 1, 1],
    vector![-1, 1, -1],
    vector![-1, -1, 1],
    vector![-1, -1, -1],
];

const SWIZZLES: [Matrix3<i64>; 6] = [
    matrix![
        1, 0, 0;
        0, 1, 0;
        0, 0, 1;
    ],
    matrix![
        1, 0, 0;
        0, 0, 1;
        0, 1, 0;
    ],
    matrix![
        0, 1, 0;
        0, 0, 1;
        1, 0, 0;
    ],
    matrix![
        0, 1, 0;
        1, 0, 0;
        0, 0, 1;
    ],
    matrix![
        0, 0, 1;
        1, 0, 0;
        0, 1, 0;
    ],
    matrix![
        0, 0, 1;
        0, 1, 0;
        1, 0, 0;
    ],
];
