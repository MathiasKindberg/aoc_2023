//! Part 1: 1898776583
//! Part 2: 1100

use ::lending_iterator::prelude::*;
use std::{collections::VecDeque, io::BufRead};

fn input() -> Vec<String> {
    let stdin = std::io::stdin();
    stdin.lock().lines().map_while(Result::ok).collect()
}

fn calculate_differences(row: &[i64]) -> Vec<i64> {
    use itertools::Itertools;

    row.iter().tuple_windows().map(|(a, b)| b - a).collect()
}

fn one(input: &[String]) {
    let now = std::time::Instant::now();
    let mut sum: i64 = 0;
    let mut input: Vec<Vec<_>> = input
        .iter()
        .map(|row| {
            vec![row
                .split_ascii_whitespace()
                .map(|value| value.parse::<i64>().expect("valid integer"))
                .collect::<Vec<_>>()]
        })
        .collect();

    for row in &mut input {
        let mut current_row = row[0].clone();
        // Could do this recursively, hmmm!
        loop {
            current_row = calculate_differences(&current_row);
            if current_row.iter().sum::<i64>() == 0 {
                break;
            } else {
                row.push(current_row.clone())
            }
        }
        sum += row.iter().map(|diffs| diffs.last().unwrap()).sum::<i64>();
    }

    println!("One: {sum} | Elapsed: {:?}", now.elapsed());
}

fn calculate_differences_vecdeque(row: &VecDeque<i64>) -> VecDeque<i64> {
    use itertools::Itertools;

    row.iter().tuple_windows().map(|(a, b)| b - a).collect()
}

fn two(input: &[String]) {
    let now = std::time::Instant::now();
    let mut sum = 0;

    let mut input: Vec<Vec<_>> = input
        .iter()
        .map(|row| {
            vec![row
                .split_ascii_whitespace()
                .map(|value| value.parse::<i64>().expect("valid integer"))
                // VecDeque to enable fast inserting from beginning of vec
                .collect::<VecDeque<_>>()]
        })
        .collect();

    for row in &mut input {
        let mut current_row = row[0].clone();
        loop {
            current_row = calculate_differences_vecdeque(&current_row);
            row.push(current_row.clone());
            if current_row.iter().sum::<i64>() == 0 {
                break;
            }
        }

        // Start from the bottom.
        row.reverse();

        // One day we will have a true nice windows_mut and lending iterator in std.
        let mut iter = row.windows_mut::<2>();
        while let Some([ref mut curr, ref mut last]) = iter.next() {
            last.push_front(last.front().unwrap() - curr.front().unwrap());
        }

        sum += row.last().unwrap().front().unwrap();
    }

    println!("Two: {sum} | Elapsed: {:?}", now.elapsed());
}

fn main() {
    let input = input();
    one(&input);
    two(&input);
}
