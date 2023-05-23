pub fn solve(input: &str) {
    let data = input
        .split("\n")
        .map(|x| x.split(": ").map(|x| x.parse::<i32>().unwrap()))
        .map(|mut x| (x.next().unwrap(), x.next().unwrap()));
    for d in 0.. {
        if data
            .clone()
            .find(|(n1, n2)| (n1 + d) % (n2 * 2 - 2) == 0)
            .is_none()
        {
            println!("{d}");
            break;
        }
    }
}
