pub fn solve(input: &str) {
    let b = input
        .split_whitespace()
        .nth(2)
        .unwrap()
        .parse::<i32>()
        .unwrap();
    let b = b * 100 + 100_000;
    let c = b + 17_000;
    let mut h = 0;
    for n in (b..=c).step_by(17) {
        if (2..(n as f64).sqrt() as i32 + 1)
            .find(|x| n % *x == 0)
            .is_some()
        {
            h += 1;
        }
    }
    println!("{h}");
}
