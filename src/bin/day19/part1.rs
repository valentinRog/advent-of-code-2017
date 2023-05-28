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

pub fn solve(input: &str) {
    let m = input
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut d = Complex { r: 0, i: 1 };
    let mut p = Complex { r: 0, i: 0 };
    p.r = m[0].iter().enumerate().find(|(_, x)| **x == '|').unwrap().0 as i32;
    let mut res = String::new();
    loop {
        match m[p.i as usize][p.r as usize] {
            ' ' => break,
            '+' => {
                d = *[d * Complex { r: 0, i: 1 }, d * Complex { r: 0, i: -1 }]
                    .iter()
                    .find(|x| m[(p.i + x.i) as usize][(p.r + x.r) as usize] != ' ')
                    .unwrap();
            }
            x if x.is_alphabetic() => res.push(x),
            _ => {}
        }
        p = p + d;
    }
    println!("{res}");
}
