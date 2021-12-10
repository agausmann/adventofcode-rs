use aoc::parsing::lines;

fn main() {
    let mut scores = lines()
        .into_iter()
        .filter(|s| corrupted(&s) == 0)
        .map(|s| incomplete(&s))
        .collect::<Vec<_>>();
    scores.sort();
    println!("{}", scores[scores.len() / 2]);
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

fn incomplete(line: &str) -> u64 {
    let mut stack: Vec<char> = Vec::new();

    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' => {
                let p = stack.pop();
                if p.is_some() && p != Some('(') {
                    panic!();
                }
            }
            ']' => {
                let p = stack.pop();
                if p.is_some() && p != Some('[') {
                    panic!();
                }
            }
            '}' => {
                let p = stack.pop();
                if p.is_some() && p != Some('{') {
                    panic!();
                }
            }
            '>' => {
                let p = stack.pop();
                if p.is_some() && p != Some('<') {
                    panic!();
                }
            }
            _ => panic!("{}", c),
        }
    }

    stack
        .iter()
        .rev()
        .map(|c| match c {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => panic!("{}", c),
        })
        .fold(0, |acc, c| 5 * acc + c)
}
