pub fn solve(input: &str) {
    let res = input
        .split("\n")
        .map(|x| {
            x.split_whitespace().map(|x| {
                let mut x = x.chars().collect::<Vec<_>>();
                x.sort();
                return x.into_iter().collect::<String>();
            })
        })
        .filter(|x| {
            x.clone()
                .collect::<std::collections::HashSet<String>>()
                .len()
                == x.clone().count()
        })
        .count();
    println!("{res}");
}
