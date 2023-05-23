pub fn solve(input: &str) {
    let res = input
        .split("\n")
        .map(|x| x.split(": ").map(|x| x.parse::<i32>().unwrap()))
        .map(|mut x| (x.next().unwrap(), x.next().unwrap()))
        .fold(0, |acc, (n1, n2)| {
            acc + if n1 % (n2 * 2 - 2) == 0 { n1 * n2 } else { 0 }
        });
    println!("{res}");
}
