pub fn solve(input: &str) {
    let res = input
        .chars()
        .zip(input.chars().cycle().skip(input.len() / 2))
        .filter(|(a, b)| a == b)
        .map(|(a, _)| a.to_digit(10).unwrap())
        .fold(0, |a, b| a + b);
    println!("{res}");
}
