use std::collections::HashMap;

pub fn solve(input: &str) {
    type BinaryOp = dyn Fn(i32, i32) -> i32;
    let op = [
        ("inc", Box::new(|a, b| a + b) as Box<BinaryOp>),
        ("dec", Box::new(|a, b| a - b) as Box<BinaryOp>),
    ]
    .into_iter()
    .collect::<HashMap<_, _>>();

    type CmpOp = dyn Fn(i32, i32) -> bool;
    let cmp = [
        ("==", Box::new(|a, b| a == b) as Box<CmpOp>),
        ("!=", Box::new(|a, b| a != b) as Box<CmpOp>),
        ("<", Box::new(|a, b| a < b) as Box<CmpOp>),
        ("<=", Box::new(|a, b| a <= b) as Box<CmpOp>),
        (">", Box::new(|a, b| a > b) as Box<CmpOp>),
        (">=", Box::new(|a, b| a >= b) as Box<CmpOp>),
    ]
    .into_iter()
    .collect::<HashMap<_, _>>();

    let mut r = input
        .split("\n")
        .map(|x| (x.split_whitespace().next().unwrap(), 0))
        .collect::<HashMap<_, _>>();

    let mut res: Option<i32> = None;
    input
        .split("\n")
        .map(|x| x.split_whitespace().collect::<Vec<_>>())
        .for_each(|x| {
            if cmp[x[5]](r[x[4]], x[6].parse::<i32>().unwrap()) {
                let n = op[x[1]](r[x[0]], x[2].parse::<i32>().unwrap());
                r.insert(x[0], n);
                match res {
                    Some(n2) => res = Some(n2.max(n)),
                    _ => res = Some(n),
                };
            }
        });
    println!("{}", res.unwrap());
}
