//! Part 1: 251287184
//! Part 2: 250757288

// Split the parts into different functions since we rely on the ordering in the
// enums for sorting which means tons of repetition....
mod part_1;
mod part_2;

use std::{collections::HashMap, io::BufRead};

fn input() -> Vec<String> {
    let stdin = std::io::stdin();
    stdin.lock().lines().map_while(Result::ok).collect()
}

fn main() {
    let input = input();
    part_1::one(&input);
    part_2::two(&input);
}
