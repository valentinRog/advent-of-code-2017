pub fn solve(input: &str) {
    let mut v = (0..256).collect::<Vec<_>>();
    let mut i = 0;
    for (ss, n) in input
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .enumerate()
    {
        let l = v.len();
        for j in 0..n / 2 {
            v.swap((i + j) % l, (i + n - 1 - j) % l);
        }
        i = (i + n + ss) % l;
    }
    println!("{}", v[0] * v[1]);
}
