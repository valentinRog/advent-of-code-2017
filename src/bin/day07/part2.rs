use std::collections::{HashMap, HashSet};

fn compute(name: &str, weights: &HashMap<String, i32>, tree: &HashMap<String, Vec<String>>) -> i32 {
    let mut w = weights[name];
    for child in &tree[name] {
        w += compute(child, weights, tree);
    }
    return w;
}

pub fn solve(input: &str) {
    let weights = input
        .replace("(", "")
        .replace(")", "")
        .split("\n")
        .map(|x| x.split_whitespace().take(2).collect::<Vec<_>>())
        .map(|x| (String::from(x[0]), x[1].parse::<i32>().unwrap()))
        .collect::<HashMap<_, _>>();
    let lines = input
        .replace(",", "")
        .split("\n")
        .map(|x| {
            x.split_whitespace()
                .filter(|x| x.chars().all(|c| c.is_alphabetic()))
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut tree = HashMap::new();
    for x in lines {
        let mut v = Vec::new();
        for e in &x[1..] {
            v.push(e.clone());
        }
        tree.insert(x[0].clone(), v);
    }
    let st = input
        .split("\n")
        .filter(|x| x.contains("->"))
        .map(|x| x.split("->").nth(1).unwrap())
        .flat_map(|x| x.split(",").map(|x| x.trim()))
        .collect::<HashSet<_>>();
    let mut name = input
        .split("\n")
        .map(|x| x.split_whitespace().nth(0).unwrap())
        .find(|x| st.get(x).is_none())
        .unwrap();
    let mut res = 0;
    loop {
        let v = &tree[name].iter().map(|x| (compute(x, &weights, &tree), x));
        if let Some((n, new_name)) = v
            .clone()
            .filter(|(x, _)| v.clone().filter(|(e, _)| e == x).count() == 1)
            .nth(0)
        {
            let target = v.clone().find(|(x, _)| *x != n).unwrap().0;
            name = new_name;
            res = weights[name] - (n - target).abs();
        } else {
            break;
        }
    }
    println!("{res}");
}
