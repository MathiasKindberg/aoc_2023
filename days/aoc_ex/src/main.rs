//! Part 1:
//! Part 2:

use std::io::BufRead;

type Input = Vec<String>;

fn input() -> Input {
    let stdin = std::io::stdin();
    stdin.lock().lines().map_while(Result::ok).collect()
}

fn one(_input: &Input) {
    let now = std::time::Instant::now();
    let sum = 0;

    println!("One: {sum} | Elapsed: {:?}", now.elapsed());
}
fn two(_input: &Input) {
    let now = std::time::Instant::now();
    let sum = 0;

    println!("Two: {sum} | Elapsed: {:?}", now.elapsed());
}

fn main() {
    let input = input();
    one(&input);
    two(&input);
}
