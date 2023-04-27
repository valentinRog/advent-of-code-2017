pub fn solve(input: &str) {
    let res = input
        .split("\n")
        .map(|x| x.split_whitespace().map(|x| x.parse::<i32>().unwrap()))
        .map(|x| x.clone().max().unwrap() - x.min().unwrap())
        .sum::<i32>();
    println!("{res}");
}
