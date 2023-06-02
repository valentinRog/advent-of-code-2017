use std::collections::HashSet;

fn compute(t: (i32, i32), remaining: HashSet<(i32, i32)>, l: i32) -> (i32, i32) {
    let mut strength = 0;
    let mut nl = l;
    for &(x, y) in remaining.iter().filter(|&&(x, y)| x == t.1 || y == t.1) {
        let mut h = remaining.clone();
        h.remove(&(x, y));
        let nt = if x == t.1 { (x, y) } else { (y, x) };
        let val = compute(nt, h, l + 1);
        if val.0 == nl {
            strength = strength.max(val.1);
        } else if val.0 > nl {
            nl = val.0;
            strength = val.1;
        }
    }
    (nl, strength + t.0 + t.1)
}

pub fn solve(input: &str) {
    let data = input
        .lines()
        .map(|x| x.split("/").map(|x| x.parse::<i32>().unwrap()))
        .map(|mut x| (x.next().unwrap(), x.next().unwrap()))
        .collect::<HashSet<_>>();

    let res = data
        .iter()
        .filter(|&&(x, _)| x == 0)
        .map(|x| {
            let mut h = data.clone();
            h.remove(x);
            compute(*x, h, 1)
        })
        .max()
        .unwrap();
    println!("{}", res.1);
}
