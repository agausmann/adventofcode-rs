use aoc::parsing::lines;

fn main() {
    for line in lines() {
        let x: i64 = line
            .chars()
            .map(|c| match c {
                '(' => 1,
                ')' => -1,
                _ => panic!("{}", c),
            })
            .sum();
        println!("{}", x);
    }
}
