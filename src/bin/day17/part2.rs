pub fn solve(input: &str) {
    let step = input.parse::<usize>().unwrap();
    let mut l = 1;
    let mut index = 0;
    let mut res = None;
    for i in 1..=50_000_000 {
        index = (index + 1 + step) % l;
        if index == 0 {
            res = Some(i);
        }
        l += 1;
    }
    println!("{}", res.unwrap());
}
