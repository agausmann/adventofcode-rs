use aoc::parsing::lines;

fn main() {
    for line in lines() {
        let mut acc: i64 = 0;
        let x = line
            .chars()
            .map(|c| match c {
                '(' => 1,
                ')' => -1,
                _ => panic!("{}", c),
            })
            .map(|k| {
                acc += k;
                acc
            })
            .position(|k| k < 0)
            .unwrap()
            + 1;
        println!("{}", x);
    }
}
