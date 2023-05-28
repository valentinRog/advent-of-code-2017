use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) {
    let weights = input
        .replace("(", "")
        .replace(")", "")
        .lines()
        .map(|x| x.split_whitespace().take(2).collect::<Vec<_>>())
        .map(|x| (String::from(x[0]), x[1].parse::<i32>().unwrap()))
        .collect::<HashMap<_, _>>();

    let tree = input
        .replace(",", "")
        .lines()
        .map(|x| {
            x.split_whitespace()
                .filter(|x| x.chars().all(|c| c.is_alphabetic()))
                .map(|x| x.to_string())
        })
        .map(|x| (x.clone().next().unwrap(), x.skip(1).collect::<Vec<_>>()))
        .collect::<HashMap<_, _>>();

    let mut name = {
        let st = input
            .lines()
            .filter(|x| x.contains("->"))
            .map(|x| x.split("->").nth(1).unwrap())
            .flat_map(|x| x.split(",").map(|x| x.trim()))
            .collect::<HashSet<_>>();
        input
            .lines()
            .map(|x| x.split_whitespace().next().unwrap())
            .find(|x| !st.contains(x))
            .unwrap()
    };

    let mut res = None;
    loop {
        let v = &tree[name].iter().map(|x| (compute(x, &weights, &tree), x));
        if let Some((n, new_name)) = v
            .clone()
            .filter(|(x, _)| v.clone().filter(|(e, _)| e == x).count() == 1)
            .next()
        {
            let target = v.clone().find(|(x, _)| *x != n).unwrap().0;
            name = new_name;
            res = Some(weights[name] - (n - target).abs());
        } else {
            break;
        }
    }
    println!("{}", res.unwrap());
}

fn compute(name: &str, weights: &HashMap<String, i32>, tree: &HashMap<String, Vec<String>>) -> i32 {
    return tree[name]
        .iter()
        .map(|x| compute(x, weights, tree))
        .sum::<i32>()
        + weights[name];
}
