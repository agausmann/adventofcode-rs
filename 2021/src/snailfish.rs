use std::fmt::{Debug, Formatter};

#[derive(Clone, PartialEq)]
pub enum Number {
    Literal(u64),
    Pair(Box<(Number, Number)>),
}

impl Debug for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.debug_inner(f, 0)
    }
}

impl Number {
    fn debug_inner(&self, f: &mut Formatter, depth: usize) -> std::fmt::Result {
        match self {
            Self::Literal(x) => write!(f, "{}", x),
            Self::Pair(tup) => {
                if depth == 4 {
                    write!(f, "X")?;
                }
                write!(f, "[")?;
                tup.0.debug_inner(f, depth + 1)?;
                write!(f, ",")?;
                tup.1.debug_inner(f, depth + 1)?;
                write!(f, "]")?;
                Ok(())
            }
        }
    }

    pub fn parse(s: &str) -> Self {
        let (number, len) = Self::parse_inner(s);
        assert_eq!(s.len(), len);
        number
    }

    fn parse_inner(mut s: &str) -> (Self, usize) {
        let mut count = 0;
        if s.starts_with('[') {
            s = &s[1..];
            count += 1;

            let (a, n) = Self::parse_inner(s);
            s = &s[n..];
            count += n;

            s = s.strip_prefix(',').unwrap();
            count += 1;

            let (b, n) = Self::parse_inner(s);
            s = &s[n..];
            count += n;

            s = s.strip_prefix(']').unwrap();
            count += 1;

            // final assignment
            drop(s);
            (Self::Pair(Box::new((a, b))), count)
        } else {
            let (mine, _) = s.split_once(&[',', ']'][..]).unwrap();
            (Self::Literal(mine.parse().unwrap()), mine.len())
        }
    }

    pub fn add(self, rhs: Self) -> Self {
        let mut result = Self::Pair(Box::new((self, rhs)));
        result.reduce();
        result
    }

    pub fn reduce(&mut self) {
        // eprintln!();
        loop {
            // eprintln!("{:?}", self);
            if !self.step_reduce() {
                break;
            }
        }
    }

    fn step_reduce(&mut self) -> bool {
        self.explode(0).is_some() || self.split()
    }

    fn explode(&mut self, depth: usize) -> Option<(Option<u64>, Option<u64>)> {
        match self {
            Self::Literal(_) => None,
            Self::Pair(tup) => {
                if depth >= 4 {
                    // eprintln!("explode");
                    let ret = Some((Some(tup.0.literal()), Some(tup.1.literal())));
                    *self = Self::Literal(0);
                    ret
                } else {
                    if let Some(mut splode) = tup.0.explode(depth + 1) {
                        tup.1.forward_explode(&mut splode.1);
                        Some(splode)
                    } else if let Some(mut splode) = tup.1.explode(depth + 1) {
                        tup.0.backward_explode(&mut splode.0);
                        Some(splode)
                    } else {
                        None
                    }
                }
            }
        }
    }

    fn forward_explode(&mut self, splode: &mut Option<u64>) {
        match self {
            Self::Literal(x) => {
                if let Some(y) = splode.take() {
                    *x += y;
                }
            }
            Self::Pair(tup) => {
                tup.0.forward_explode(splode);
            }
        }
    }

    fn backward_explode(&mut self, splode: &mut Option<u64>) {
        match self {
            Self::Literal(x) => {
                if let Some(y) = splode.take() {
                    *x += y;
                }
            }
            Self::Pair(tup) => {
                tup.1.backward_explode(splode);
            }
        }
    }

    fn split(&mut self) -> bool {
        match self {
            &mut Self::Literal(x) => {
                if x >= 10 {
                    // eprintln!("split");
                    let down = x / 2;
                    let up = (x - 1) / 2 + 1;
                    *self = Self::Pair(Box::new((Self::Literal(down), Self::Literal(up))));
                    true
                } else {
                    false
                }
            }
            Self::Pair(tup) => tup.0.split() || tup.1.split(),
        }
    }

    fn literal(&self) -> u64 {
        match self {
            Self::Literal(x) => *x,
            _ => panic!("illegal explode"),
        }
    }

    pub fn magnitude(&self) -> u64 {
        match self {
            Self::Literal(x) => *x,
            Self::Pair(tup) => 3 * tup.0.magnitude() + 2 * tup.1.magnitude(),
        }
    }
}
