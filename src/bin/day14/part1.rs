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

pub fn solve(input: &str) {
    for i in 0..128 {
        let hash = hash(&format!("{input}-{i}"));
        let hash = hash
            .chars()
            .map(|x| i32::from_str_radix(&x.to_string(), 2).unwrap())
            .map(|x| format!("{:04x}", x))
            .collect::<String>();
    }
}
