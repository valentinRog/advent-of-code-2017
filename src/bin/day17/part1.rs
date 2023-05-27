use std::collections::VecDeque;

pub fn solve(input: &str) {
    let step = input.parse::<usize>().unwrap();
    let mut v = VecDeque::from([0]);
    let mut index = 0;
    for i in 1.. {
        index = (index + 1 + step) % v.len();
        if i == 2017 {
            println!("{}", v[(index + 1) % v.len()]);
            break;
        }
        v.insert(index + 1, i);
    }
}
