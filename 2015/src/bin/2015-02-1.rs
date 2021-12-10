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

    println!("{}", boxes.into_iter().map(wrapping_paper).sum::<u64>());
}

fn wrapping_paper((l, w, h): (u64, u64, u64)) -> u64 {
    let a = l * w;
    let b = w * h;
    let c = l * h;
    2 * (a + b + c) + a.min(b).min(c)
}
