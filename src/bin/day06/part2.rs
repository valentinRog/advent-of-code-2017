use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) {
    let mut v: Vec<_> = input
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let mut st = HashSet::new();
    let mut mp = HashMap::new();
    for n in 0.. {
        if st.get(&v).is_some() {
            println!("{}", n - mp[&v]);
            break;
        }
        st.insert(v.clone());
        mp.insert(v.clone(), n);
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
