pub fn solve(input: &str) {
    let mut v: Vec<_> = input
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    let mut i: i32 = 0;
    let mut k = 0;
    while i >= 0 && i < v.len() as i32 {
        let n = &mut v[i as usize];
        i += *n;
        *n += if *n < 3 { 1 } else { -1 };
        k += 1;
    }
    println!("{k}");
}
