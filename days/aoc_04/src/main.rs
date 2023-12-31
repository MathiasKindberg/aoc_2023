//! Part 1: 25651
//! Part 2: 19499881

use std::{collections::HashSet, io::BufRead};

fn input() -> Vec<String> {
    let stdin = std::io::stdin();
    stdin.lock().lines().map_while(Result::ok).collect()
}

fn process_input(input: &[String]) -> Vec<(HashSet<u64>, Vec<u64>)> {
    use itertools::Itertools;

    input
        .iter()
        .map(|row| row.split(": ").collect_tuple().unwrap())
        .map(|(_, input)| input.split(" | ").collect_tuple().unwrap())
        .map(|(winning, your): (&str, &str)| {
            (
                winning
                    .split_ascii_whitespace()
                    .filter_map(|num| num.parse().ok())
                    .collect(),
                your.split_ascii_whitespace()
                    .filter_map(|num| num.parse().ok())
                    .collect(),
            )
        })
        .collect()
}

fn one(input: &[String]) {
    let now = std::time::Instant::now();

    let input = process_input(input);

    let sum: u64 = input
        .iter()
        .filter_map(|(winning, yours)| {
            let matching: u32 = yours
                .iter()
                .filter(|num| winning.contains(num))
                .count()
                .try_into()
                .unwrap();

            match matching {
                1.. => Some(matching),
                _ => None,
            }
        })
        .map(|matching| 2_u64.pow(matching - 1))
        .sum();

    println!("One: {sum} | Elapsed: {:?}", now.elapsed());
}

fn two(input: &[String]) {
    let now = std::time::Instant::now();
    let input = process_input(input);
    let mut copies: Vec<u64> = vec![1; input.len()];

    for (idx, (winning, yours)) in input.iter().enumerate() {
        let matching = yours.iter().filter(|num| winning.contains(num)).count();

        for copies_idx in 1..=matching {
            copies[idx + copies_idx] += copies[idx];
        }
    }

    let sum: u64 = copies.iter().sum();
    println!("Two: {sum} | Elapsed: {:?}", now.elapsed());
}

fn main() {
    let input = input();
    one(&input);
    two(&input);
}
