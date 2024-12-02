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
    let mut s = std::collections::HashSet::<Complex>::new();
    for _ in 1..n {
        z = z + d;
        if !s.contains(&(z + d * Complex(0, 1))) {
            d = Complex(0, 1) * d;
        }
        s.insert(z);
    }
    println!("{}", z.0.abs() + z.1.abs());
}
