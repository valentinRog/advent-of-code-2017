use std::collections::HashMap;

fn rot(s: &Vec<char>) -> Vec<char> {
    let w = (s.len() as f64).sqrt() as usize;
    let mut res = Vec::new();
    for i in 0..w {
        for j in 0..w {
            res.push(s[(w - 1 - j) * w + i]);
        }
    }
    res
}

fn vflip(s: &Vec<char>) -> Vec<char> {
    let w = (s.len() as f64).sqrt() as usize;
    let mut res = Vec::new();
    for i in 0..w {
        for j in 0..w {
            res.push(s[(w - 1 - i) * w + j]);
        }
    }
    res
}

pub fn solve(input: &str) {
    let mut h = HashMap::new();
    input
        .replace("/", "")
        .lines()
        .map(|x| x.split(" => "))
        .for_each(|mut x| {
            let k = x.next().unwrap().chars().collect::<Vec<_>>();
            let v = x.next().unwrap().chars().collect::<Vec<_>>();
            let mut s = k.clone();
            for _ in 0..4 {
                s = rot(&s);
                h.insert(s.clone(), v.clone());
                h.insert(vflip(&s), v.clone());
            }
        });
    let mut map = ".#...####".chars().collect::<Vec<_>>();
    for _ in 0..18 {
        let w = (map.len() as f64).sqrt() as usize;
        let cell_w = if w % 2 == 0 { 2 } else { 3 };
        let mut grid = Vec::new();
        for i in 0..w / cell_w {
            for j in 0..w / cell_w {
                let mut cell = Vec::new();
                for ii in 0..cell_w {
                    for jj in 0..cell_w {
                        cell.push(map[(i * cell_w + ii) * w + j * cell_w + jj]);
                    }
                }
                grid.push(h[&cell].clone());
            }
        }
        map.clear();
        let n_cell_w = w / cell_w;
        let cell_w = cell_w + 1;
        for i in 0..cell_w * n_cell_w {
            for j in 0..cell_w * n_cell_w {
                map.push(
                    grid[(i / cell_w) * n_cell_w + j / cell_w][(i % cell_w) * cell_w + j % cell_w],
                );
            }
        }
    }
    let res = map.iter().filter(|&&x| x == '#').count();
    println!("{res}");
}
