use std::{collections::HashMap, mem::swap};

use aoc::parsing::lines;

fn main() {
    let lines = lines();
    let p1_start = lines[0]
        .split_once(": ")
        .map(|tup| tup.1)
        .unwrap_or(&lines[0])
        .parse()
        .unwrap();
    let p2_start = lines[1]
        .split_once(": ")
        .map(|tup| tup.1)
        .unwrap_or(&lines[1])
        .parse()
        .unwrap();

    let mut states: HashMap<Status, u64> = HashMap::new();
    states.insert(Status::Playing(State::start(p1_start, p2_start)), 1);
    let mut next = HashMap::new();

    loop {
        // for state in &states {
        //     println!("{:?}", state);
        // }
        // println!("{:?}", next);
        // println!();

        let mut finished = true;
        next.clear();
        for (state, count) in states.drain() {
            if state.is_won() {
                *next.entry(state).or_insert(0) += count;
            } else {
                finished = false;
                let die_rolls = (1..=3)
                    .flat_map(|a| (1..=3).flat_map(move |b| (1..=3).map(move |c| a + b + c)));
                for die in die_rolls {
                    let mut next_state = state;
                    next_state.step(die);
                    *next.entry(next_state).or_insert(0) += count;
                }
            }
        }

        swap(&mut states, &mut next);
        if finished {
            break;
        }
    }
    println!(
        "{}",
        states[&Status::Won(PlayerId::P1)].max(states[&Status::Won(PlayerId::P2)])
    );
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum PlayerId {
    P1,
    P2,
}

impl PlayerId {
    fn next_turn(self) -> Self {
        match self {
            Self::P1 => Self::P2,
            Self::P2 => Self::P1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Status {
    Playing(State),
    Won(PlayerId),
}

impl Status {
    fn is_won(&self) -> bool {
        match self {
            Self::Playing(..) => false,
            Self::Won(..) => true,
        }
    }

    fn step(&mut self, die: u16) {
        match self {
            Self::Playing(state) => match state.step(die) {
                Some(winner) => {
                    *self = Status::Won(winner);
                }
                None => {}
            },
            Self::Won(..) => panic!("cannot continue playing a won game"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    next: PlayerId,
    p1: Player,
    p2: Player,
}

impl State {
    fn start(p1_start: u16, p2_start: u16) -> Self {
        Self {
            next: PlayerId::P1,
            p1: Player {
                score: 0,
                position: p1_start % 10,
            },
            p2: Player {
                score: 0,
                position: p2_start % 10,
            },
        }
    }

    fn step(&mut self, die: u16) -> Option<PlayerId> {
        let player = match self.next {
            PlayerId::P1 => &mut self.p1,
            PlayerId::P2 => &mut self.p2,
        };
        player.play(die);
        let winner = if player.score >= 21 {
            Some(self.next)
        } else {
            None
        };
        self.next = self.next.next_turn();
        winner
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Player {
    score: u16,
    position: u16,
}

impl Player {
    fn play(&mut self, steps: u16) {
        self.position = (self.position + steps) % 10;
        if self.position == 0 {
            self.score += 10;
        } else {
            self.score += self.position;
        }
    }
}
