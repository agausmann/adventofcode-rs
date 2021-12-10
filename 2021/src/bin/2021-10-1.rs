use aoc::parsing::lines;

fn main() {
    let lines = lines();

    println!("{}", lines.into_iter().map(|s| corrupted(&s)).sum::<u64>());
}

fn corrupted(line: &str) -> u64 {
    let mut stack: Vec<char> = Vec::new();

    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' => {
                let p = stack.pop();
                if p.is_some() && p != Some('(') {
                    return 3;
                }
            }
            ']' => {
                let p = stack.pop();
                if p.is_some() && p != Some('[') {
                    return 57;
                }
            }
            '}' => {
                let p = stack.pop();
                if p.is_some() && p != Some('{') {
                    return 1197;
                }
            }
            '>' => {
                let p = stack.pop();
                if p.is_some() && p != Some('<') {
                    return 25137;
                }
            }
            _ => panic!("{}", c),
        }
    }
    0
}
