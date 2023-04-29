pub fn solve(input: &str) {
    let res = input
        .split("\n")
        .map(|x| x.split_whitespace())
        .filter(|x| {
            x.clone().collect::<std::collections::HashSet<&str>>().len() == x.clone().count()
        })
        .count();
    println!("{res}");
}
