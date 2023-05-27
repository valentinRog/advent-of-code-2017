enum Op {
    S(usize),
    X(usize, usize),
    P(char, char),
}

pub fn solve(input: &str) {
    let data = input.split(",").map(|x| match x.chars().next().unwrap() {
        's' => Op::S(x[1..].parse().unwrap()),
        'x' => {
            let mut arr = x[1..].split("/");
            Op::X(
                arr.next().unwrap().parse().unwrap(),
                arr.next().unwrap().parse().unwrap(),
            )
        }
        'p' => Op::P(x.chars().nth(1).unwrap(), x.chars().nth(3).unwrap()),
        _ => panic!(),
    });

    let mut v = (b'a'..=b'p').map(|x| x as char).collect::<Vec<_>>();
    let init = v.clone();
    let mut i = 0;
    const N: i32 = 1_000_000_000;
    while i < N {
        for op in data.clone() {
            match op {
                Op::S(a) => {
                    v.rotate_right(a);
                }
                Op::X(a, b) => {
                    v.swap(a, b);
                }
                Op::P(a, b) => {
                    let a = v.iter().position(|x| *x == a).unwrap();
                    let b = v.iter().position(|x| *x == b).unwrap();
                    v.swap(a, b);
                }
            }
        }
        i += 1;
        if v == init {
            i = N - N % i;
        }
    }
    println!("{}", v.iter().collect::<String>());
}
