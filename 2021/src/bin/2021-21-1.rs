use aoc::parsing::lines;

fn main() {
    let lines = lines();
    let p1_start: u64 = lines[0]
        .split_once(": ")
        .map(|tup| tup.1)
        .unwrap_or(&lines[0])
        .parse()
        .unwrap();
    let p2_start: u64 = lines[1]
        .split_once(": ")
        .map(|tup| tup.1)
        .unwrap_or(&lines[1])
        .parse()
        .unwrap();
    let win = 1000;

    let mut p1 = Player {
        score: 0,
        position: p1_start,
    };
    let mut p2 = Player {
        score: 0,
        position: p2_start,
    };
    let mut die_state = 0;
    let mut rolls = 0;
    let mut roll = || {
        die_state = (die_state + 1) % 100;
        rolls += 1;
        if die_state == 0 {
            100
        } else {
            die_state
        }
    };
    let loser_score;
    loop {
        p1.play(roll() + roll() + roll());
        if p1.score >= win {
            loser_score = p2.score;
            break;
        }
        p2.play(roll() + roll() + roll());
        if p2.score >= win {
            loser_score = p1.score;
            break;
        }
    }
    println!("{}", loser_score * rolls);
}

struct Player {
    score: u64,
    position: u64,
}

impl Player {
    fn play(&mut self, steps: u64) {
        self.position = (self.position + steps) % 10;
        if self.position == 0 {
            self.score += 10;
        } else {
            self.score += self.position;
        }
    }
}
