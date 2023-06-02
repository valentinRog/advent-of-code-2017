use std::collections::HashMap;

struct Ins {
    write: bool,
    left: bool,
    next: char,
}

pub fn solve(input: &str) {
    let mut tape = HashMap::from([(0, false)]);
    let mut bp = HashMap::new();
    for state in input.split("\n\n").skip(1) {
        let s = state.lines().next().unwrap().chars().nth(9).unwrap();
        let arr = state
            .split("\n")
            .map(|x| x.trim())
            .filter(|x| x.starts_with("-"))
            .map(|x| x.split_whitespace().rev().next().unwrap())
            .collect::<Vec<_>>();
        let h = [false, true]
            .iter()
            .enumerate()
            .map(|(i, b)| {
                (
                    *b,
                    Ins {
                        write: arr[0 + i * 3].chars().next().unwrap() == '1',
                        left: arr[1 + i * 3].starts_with("left"),
                        next: arr[2 + i * 3].chars().next().unwrap(),
                    },
                )
            })
            .collect::<HashMap<_, _>>();
        bp.insert(s, h);
    }

    let mut state = input
        .split_whitespace()
        .nth(3)
        .unwrap()
        .chars()
        .next()
        .unwrap();
    let mut i = 0;
    let n = input
        .lines()
        .nth(1)
        .unwrap()
        .split_whitespace()
        .rev()
        .nth(1)
        .unwrap()
        .parse::<i32>()
        .unwrap();
    for _ in 0..n {
        let ins = &bp[&state][tape.entry(i).or_insert(false)];
        *tape.get_mut(&i).unwrap() = ins.write;
        i += if ins.left { -1 } else { 1 };
        state = ins.next;
    }
    let res = tape.iter().filter(|(_, &x)| x).count();
    println!("{res}");
}
