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
            .take_while(|&k| k >= 0)
            .count()
            + 1;
        println!("{}", x);
    }
}
