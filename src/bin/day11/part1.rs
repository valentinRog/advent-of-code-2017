use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Array2d(i32, i32);

impl std::ops::Add for Array2d {
    type Output = Array2d;
    fn add(self, rhs: Array2d) -> Array2d {
        Array2d(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Array2d {
    fn sum(&self) -> i32 {
        self.0 + self.1
    }
}

pub fn solve(input: &str) {
    let dir = HashMap::from([
        ("n", Array2d(0, -1)),
        ("ne", Array2d(-1, 0)),
        ("se", Array2d(-1, 1)),
        ("s", Array2d(0, 1)),
        ("sw", Array2d(1, 0)),
        ("nw", Array2d(1, -1)),
    ]);

    let res = input.split(",").fold(Array2d(0, 0), |acc, d| acc + dir[d]);
    let res = [res.0, res.1, res.sum()]
        .iter()
        .map(|x| x.abs())
        .max()
        .unwrap();
    println!("{res}");
}
