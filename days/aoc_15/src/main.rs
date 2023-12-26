//! Part 1:
//! Part 2:

use std::io::BufRead;

fn input() -> Vec<String> {
    let stdin = std::io::stdin();
    stdin.lock().lines().map_while(Result::ok).collect()
}

fn hash(mut current_value: u64, c: char) -> u64 {
    current_value += c as u64;
    current_value *= 17;
    current_value %= 256;
    current_value
}

fn encode(s: &str) -> u64 {
    s.chars().fold(0, hash)
}

fn one(input: &[String]) {
    let now = std::time::Instant::now();
    let sum: u64 = input[0].split(',').map(encode).sum();
    println!("One: {sum} | Elapsed: {:?}", now.elapsed());
}
fn two(_input: &[String]) {
    let now = std::time::Instant::now();
    let sum = 0;

    println!("Two: {sum} | Elapsed: {:?}", now.elapsed());
}

fn main() {
    let input = input();
    assert_eq!(
        input.len(),
        1,
        "Ignoring new lines means the input is one line"
    );
    one(&input);
    two(&input);
}
