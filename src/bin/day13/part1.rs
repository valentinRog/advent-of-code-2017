pub fn solve(input: &str) {
    let res = input
        .split("\n")
        .map(|x| x.split(": ").map(|x| x.parse::<i32>().unwrap()))
        .fold(0, |acc, mut x| {
            let (n1, n2) = (x.next().unwrap(), x.next().unwrap());
            acc + if n1 % (n2 * 2 - 2) == 0 {
                n1 * n2
            } else {
                0
            }
        });
    println!("{res}");
}
