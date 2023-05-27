pub fn solve(input: &str) {
    let mut it = input.lines().map(|line| {
        line.split_whitespace()
            .nth(4)
            .unwrap()
            .parse::<u64>()
            .unwrap()
    });
    let (mut a, mut b) = (it.next().unwrap(), it.next().unwrap());
    let res = (0..40_000_000).fold(0, |acc, _| {
        a = (a * 16807) % std::i32::MAX as u64;
        b = (b * 48271) % std::i32::MAX as u64;
        acc + (a & 0xffff == b & 0xffff) as i32
    });
    println!("{res}");
}
