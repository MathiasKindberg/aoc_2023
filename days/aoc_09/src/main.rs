//! Part 1: 1898776583
//! Part 2:

use std::io::BufRead;

fn input() -> Vec<String> {
    let stdin = std::io::stdin();
    stdin.lock().lines().map_while(Result::ok).collect()
}

fn calculate_differences(row: Vec<i64>) -> Vec<i64> {
    use itertools::Itertools;
    row.into_iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect()
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
            current_row = calculate_differences(current_row);
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
fn two(_input: &[String]) {
    let now = std::time::Instant::now();
    let sum = 0;

    println!("Two: {sum} | Elapsed: {:?}", now.elapsed());
}

fn main() {
    let input = input();
    one(&input);
    two(&input);
}
