use std::collections::HashSet;

pub fn solve(input: &str) {
    let mut v: Vec<_> = input
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let mut s = HashSet::new();
    for n in 0.. {
        if s.get(&v).is_some() {
            println!("{n}");
            break;
        }
        s.insert(v.clone());
        let (maxi, max) = v
            .iter()
            .enumerate()
            .max_by(|(i1, x), (i2, y)| if x == y { i2.cmp(i1) } else { x.cmp(y) })
            .unwrap();
        let l = v.len();
        let (d, m) = (max / l, max % l);
        v.iter_mut().for_each(|x| *x += d);
        v[maxi] = 0;
        for i in maxi + 1..maxi + 1 + m {
            v[i % l] += 1;
        }
    }
}
