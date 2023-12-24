//! Part 1:
//! Part 2:

use std::io::BufRead;

#[derive(Debug, Clone)]
enum Ground {
    Round,
    Cube,
    Empty,
}

fn input() -> Vec<Vec<Ground>> {
    let stdin = std::io::stdin();
    stdin
        .lock()
        .lines()
        .map_while(Result::ok)
        .map(|row| {
            row.chars()
                .map(|c| match c {
                    'O' => Ground::Round,
                    '#' => Ground::Cube,
                    '.' => Ground::Empty,
                    err => unreachable!("Unknown input: {err:?}"),
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn one(input: Vec<Vec<Ground>>) {
    let now = std::time::Instant::now();
    let sum = 0;

    println!("One: {sum} | Elapsed: {:?}", now.elapsed());
}
fn two(_input: Vec<Vec<Ground>>) {
    let now = std::time::Instant::now();
    let sum = 0;

    println!("Two: {sum} | Elapsed: {:?}", now.elapsed());
}

fn main() {
    let input = input();
    one(input.clone());
    two(input);
}
