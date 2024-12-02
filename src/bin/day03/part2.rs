#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
struct Complex(i32, i32);

impl std::ops::Add for Complex {
    type Output = Complex;
    fn add(self, rhs: Complex) -> Complex {
        Complex(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::Mul for Complex {
    type Output = Complex;
    fn mul(self, rhs: Complex) -> Complex {
        Complex(
            self.0 * rhs.0 - self.1 * rhs.1,
            self.0 * rhs.1 + self.1 * rhs.0,
        )
    }
}

pub fn solve(input: &str) {
    let n = input.parse::<i32>().unwrap();
    let mut z = Complex(0, 0);
    let mut d = Complex(1, 0);
    let mut s = std::collections::HashMap::<Complex, i32>::new();
    let mut v = 1;
    s.insert(z, v);
    while v < n {
        z = z + d;
        v = 0;
        for z2 in [
            Complex(1, 0),
            Complex(1, 1),
            Complex(1, -1),
            Complex(0, 1),
            Complex(0, -1),
            Complex(-1, 0),
            Complex(-1, 1),
            Complex(-1, -1),
        ] {
            v += s.get(&(z2 + z)).unwrap_or(&0);
        }
        s.insert(z, v);
        if s.get(&(z + d * Complex(0, 1))).is_none() {
            d = Complex(0, 1) * d;
        }
    }
    println!("{v}");
}
