use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
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

pub fn solve(input: &str) {
    let mut h = HashSet::new();
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                h.insert(Complex {
                    r: j as i32,
                    i: i as i32,
                });
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
    for _ in 0..10000 {
        if h.contains(&p) {
            d = d * Complex { r: 0, i: 1 };
            h.remove(&p);
        } else {
            d = d * Complex { r: 0, i: -1 };
            h.insert(p);
            res += 1;
        }
        p = p + d;
    }
    println!("{res}");
}
