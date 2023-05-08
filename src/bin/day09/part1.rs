pub fn solve(input: &str) {
    let res = input
        .chars()
        .fold((false, false, 1, 0), |(skip, garbage, z, res), c| {
            match (skip, garbage, c) {
                (true, _, _) => (false, garbage, z, res),
                (_, _, '!') => (true, garbage, z, res),
                (_, true, '>') => (skip, false, z, res),
                (_, false, '<') => (skip, true, z, res),
                (_, true, _) => (skip, garbage, z, res),
                (_, false, '{') => (skip, garbage, z + 1, res + z),
                (_, false, '}') => (skip, garbage, z - 1, res),
                (_, _, _) => (skip, garbage, z, res),
            }
        })
        .3;
    println!("{}", res);
}
