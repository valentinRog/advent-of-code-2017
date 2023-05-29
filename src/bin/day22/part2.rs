use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Complex {
    r: i32,
    i: i32,
}

impl std::ops::Add for Complex {
    type Output = Complex;
    fn add(self, rhs: Complex) -> Complex {
        Complex {
            r: self.r + rhs.r,
            i: self.i + rhs.i,
        }
    }
}

impl std::ops::Mul for Complex {
    type Output = Complex;
    fn mul(self, rhs: Complex) -> Complex {
        Complex {
            r: self.r * rhs.r - self.i * rhs.i,
            i: self.r * rhs.i + self.i * rhs.r,
        }
    }
}

impl std::hash::Hash for Complex {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.r.hash(state);
        self.i.hash(state);
    }
}

enum State {
    Weakened,
    Infected,
    Flagged,
}

pub fn solve(input: &str) {
    let mut h = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                h.insert(
                    Complex {
                        r: j as i32,
                        i: i as i32,
                    },
                    State::Infected,
                );
            }
        }
    }
    let w = input.lines().count();
    let mut p = Complex {
        r: w as i32 / 2,
        i: w as i32 / 2,
    };
    let mut res = 0;
    let mut d = Complex { r: 0, i: -1 };
    for _ in 0..10000000 {
        if h.contains_key(&p) {
            match h[&p] {
                State::Weakened => {
                    h.insert(p, State::Infected);
                    res += 1;
                }
                State::Infected => {
                    h.insert(p, State::Flagged);
                    d = d * Complex { r: 0, i: 1 };
                }
                State::Flagged => {
                    h.remove(&p);
                    d = d * Complex { r: 0, i: 1 } * Complex { r: 0, i: 1 };
                }
            }
        } else {
            d = d * Complex { r: 0, i: -1 };
            h.insert(p, State::Weakened);
        }
        p = p + d;
    }
    println!("{res}");
}
