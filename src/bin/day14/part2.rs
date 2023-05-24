use std::collections::HashSet;
use std::i32;

fn hash(input: &str) -> String {
    let mut v = (0..256).collect::<Vec<_>>();
    let mut i = 0;
    let data = input
        .bytes()
        .chain([17, 31, 73, 47, 23].iter().cloned())
        .map(|x| x as usize)
        .collect::<Vec<_>>();
    for k in 0..64 {
        for (ss, n) in data.iter().clone().enumerate() {
            let l = v.len();
            for j in 0..n / 2 {
                v.swap((i + j) % l, (i + n - 1 - j) % l);
            }
            i = (i + n + ss + k * data.len()) % l;
        }
    }
    v.chunks(16)
        .map(|chunk| format!("{:02x}", chunk.iter().fold(0, |n, x| n ^ x)))
        .collect::<String>()
}

const W: i32 = 128;

fn dfs(p: i32, h: &mut HashSet<i32>) {
    if !h.contains(&p) {
        return;
    }
    h.remove(&p);
    dfs(p - W, h);
    dfs(p + W, h);
    if p % W != 0 {
        dfs(p - 1, h);
    }
    if p % W != W - 1 {
        dfs(p + 1, h);
    }
}

pub fn solve(input: &str) {
    let mut h = HashSet::new();
    for i in 0..128 {
        hash(&format!("{input}-{i}"))
            .chars()
            .map(|x| i32::from_str_radix(&x.to_string(), 16).unwrap())
            .map(|x| format!("{:04b}", x))
            .collect::<String>()
            .chars()
            .enumerate()
            .for_each(|(j, x)| {
                if x == '1' {
                    h.insert(i * W + j as i32);
                }
            })
    }
    let mut res = 0;
    while h.len() > 0 {
        dfs(*h.iter().next().unwrap(), &mut h);
        res += 1;
    }
    println!("{res}");
}
