use std::collections::{HashMap, HashSet};

fn compute(n: i32, connections: &HashMap<i32, Vec<i32>>, visited: &mut HashSet<i32>) -> i32 {
    let mut res = 0;
    for x in &connections[&n] {
        if !visited.contains(x) {
            visited.insert(*x);
            res += 1 + compute(*x, connections, visited);
        }
    }
    res
}

pub fn solve(input: &str) {
    let connections = input
        .replace("<->", "")
        .replace(",", "")
        .split("\n")
        .map(|x| {
            x.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .fold(HashMap::new(), |mut m, x| {
            m.insert(x[0], x[1..].to_vec());
            m
        });
    let res = compute(0, &connections, &mut HashSet::new());
    println!("{res}")
}
