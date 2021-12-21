use aoc::parsing::lines;
use aoc2021::snailfish::Number;

fn main() {
    let numbers: Vec<Number> = lines().into_iter().map(|s| Number::parse(&s)).collect();

    let mut max = 0;
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            if i == j {
                continue;
            }
            let sum = numbers[i].clone().add(numbers[j].clone());
            max = max.max(sum.magnitude());
        }
    }
    println!("{}", max);
}
