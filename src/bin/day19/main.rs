mod part1;
mod part2;

use std::io::{self, Read};
fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input = &input;
    part1::solve(input);
    part2::solve(input);
}
