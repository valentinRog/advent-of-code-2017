use std::collections::HashMap;

fn get_val(k: &str, rs: &HashMap<&str, i64>) -> i64 {
    k.parse::<i64>().unwrap_or(*rs.get(k).unwrap_or(&0))
}

pub fn solve(input: &str) {
    let mut i = 0i32;
    let lines = input.lines().collect::<Vec<_>>();
    let mut rs = HashMap::new();
    let mut res = 0;
    while i >= 0 && i < lines.len() as i32 {
        let mut arr = lines[i as usize].split_whitespace();
        let ins = arr.next().unwrap();
        let x = arr.next().unwrap();
        let y = arr.next();
        match ins {
            "set" => {
                rs.insert(x, get_val(y.unwrap(), &rs));
            }
            "sub" => {
                rs.insert(x, rs.get(x).unwrap_or(&0) - get_val(y.unwrap(), &rs));
            }
            "mul" => {
                rs.insert(x, rs.get(x).unwrap_or(&0) * get_val(y.unwrap(), &rs));
                res += 1;
            }
            "jnz" => {
                if get_val(x, &rs) != 0 {
                    i += (get_val(y.unwrap(), &rs) - 1) as i32;
                }
            }
            _ => panic!(),
        }
        i += 1;
    }
    println!("{res}");
}
