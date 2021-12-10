use aoc::parsing::lines;

fn main() {
    let boxes: Vec<(u64, u64, u64)> = lines()
        .into_iter()
        .map(|line| {
            let mut nums = line.split('x').map(|s| s.parse().unwrap());
            let l = nums.next().unwrap();
            let w = nums.next().unwrap();
            let h = nums.next().unwrap();
            (l, w, h)
        })
        .collect();

    println!("{}", boxes.into_iter().map(ribbon).sum::<u64>());
}

fn ribbon((l, w, h): (u64, u64, u64)) -> u64 {
    2 * (l + w).min(w + h).min(l + h) + l * w * h
}
