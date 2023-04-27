pub fn solve(input: &str) {
    let res = input
        .split("\n")
        .map(|x| x.split_whitespace().map(|x| x.parse::<i32>().unwrap()))
        .map(|line| {
            for x in line.clone() {
                for y in line.clone() {
                    if x != y && x % y == 0 {
                        return x / y;
                    }
                }
            }
            panic!();
        })
        .sum::<i32>();
    println!("{res}");
}
