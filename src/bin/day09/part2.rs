pub fn solve(input: &str) {
    let res = input
        .chars()
        .fold((false, false, 0), |(skip, garbage, res), c| {
            match (skip, garbage, c) {
                (true, _, _) => (false, garbage, res),
                (_, _, '!') => (true, garbage, res),
                (_, true, '>') => (skip, false, res),
                (_, false, '<') => (skip, true, res),
                (_, true, _) => (skip, garbage, res + 1),
                (_, _, _) => (skip, garbage, res),
            }
        })
        .2;
    println!("{}", res);
}
