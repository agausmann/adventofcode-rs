use aoc::parsing::lines;
use aoc2021::snailfish::Number;

fn main() {
    let numbers: Vec<Number> = lines().into_iter().map(|s| Number::parse(&s)).collect();
    let mut acc = numbers[0].clone();
    for term in &numbers[1..] {
        acc = acc.add(term.clone());
    }
    println!("{}", acc.magnitude());
}
