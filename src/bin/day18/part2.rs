use std::collections::{HashMap, VecDeque};

fn get_val(k: &str, rs: &HashMap<&str, i64>) -> i64 {
    k.parse::<i64>().unwrap_or(*rs.get(k).unwrap_or(&0))
}

pub fn solve(input: &str) {
    let lines = input.lines().collect::<Vec<_>>();
    let mut idx = [0i32, 0];
    let mut rs = [HashMap::new(), HashMap::from([("p", 1)])];
    let mut qs = [VecDeque::new(), VecDeque::new()];
    let mut locks = [false, false];
    let mut res = 0;
    while !locks.iter().all(|x| *x) {
        for i in 0..2 {
            let mut arr = lines[idx[i] as usize].split_whitespace();
            let ins = arr.next().unwrap();
            let x = arr.next().unwrap();
            let y = arr.next();
            match ins {
                "snd" => {
                    qs[(i + 1) % 2].push_back(get_val(x, &rs[i]));
                    if i == 1 {
                        res += 1;
                    }
                }
                "set" => {
                    rs[i].insert(x, get_val(y.unwrap(), &rs[i]));
                }
                "add" => {
                    rs[i].insert(x, rs[i].get(x).unwrap_or(&0) + get_val(y.unwrap(), &rs[i]));
                }
                "mul" => {
                    rs[i].insert(x, rs[i].get(x).unwrap_or(&0) * get_val(y.unwrap(), &rs[i]));
                }
                "mod" => {
                    rs[i].insert(x, rs[i].get(x).unwrap_or(&0) % get_val(y.unwrap(), &rs[i]));
                }
                "rcv" => {
                    if let Some(val) = qs[i].pop_front() {
                        rs[i].insert(x, val);
                    } else {
                        locks[i] = true;
                        continue;
                    }
                }
                "jgz" => {
                    if get_val(x, &rs[i]) > 0 {
                        idx[i] += (get_val(y.unwrap(), &rs[i]) - 1) as i32;
                    }
                }
                _ => panic!(),
            }
            idx[i] += 1;
        }
    }
    println!("{res}");
}
