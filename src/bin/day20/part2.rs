#[derive(Debug, Clone, Copy)]
struct Vec3(i32, i32, i32);

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Vec3) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}

impl Vec3 {
    fn from_vec(v: &Vec<i32>) -> Self {
        Self(v[0], v[1], v[2])
    }
}

#[derive(Debug, Clone, Copy)]
struct P {
    p: Vec3,
    v: Vec3,
    a: Vec3,
}

pub fn solve(input: &str) {
    let mut data = input
        .lines()
        .map(|line| {
            line.split(", ")
                .map(|x| {
                    (&x[3..x.len() - 1])
                        .split(",")
                        .map(|x| x.parse::<i32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .map(|x| P {
            p: Vec3::from_vec(&x[0]),
            v: Vec3::from_vec(&x[1]),
            a: Vec3::from_vec(&x[2]),
        })
        .collect::<Vec<_>>();
    for _ in 0..1000 {
        for p in &mut data {
            p.v += p.a;
            p.p += p.v;
        }
        data = data
            .iter()
            .filter(|p| data.iter().filter(|q| p.p == q.p).count() == 1)
            .map(|p| *p)
            .collect::<Vec<_>>();
    }
    println!("{}", data.len());
}
