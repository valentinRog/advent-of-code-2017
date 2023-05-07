use std::collections::HashSet;

pub fn solve(input: &str) {
    let st = input
        .split("\n")
        .filter(|x| x.contains("->"))
        .map(|x| x.split("->").nth(1).unwrap())
        .flat_map(|x| x.split(",").map(|x| x.trim()))
        .collect::<HashSet<_>>();
    let res = input
        .split("\n")
        .map(|x| x.split_whitespace().next().unwrap())
        .find(|x| !st.contains(x))
        .unwrap();
    println!("{res}");
}
