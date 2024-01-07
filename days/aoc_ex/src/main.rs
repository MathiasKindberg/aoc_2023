//! Part 1:
//! Part 2:

use std::io::BufRead;

type Input = Vec<String>;

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

fn parse(input: &[String]) -> Input {
    input.iter().map(|row| row.to_owned()).collect()
}

fn main() {
    // Keep input owned by main function to allow nifty references.
    let stdin = std::io::stdin();
    let input: Vec<String> = stdin.lock().lines().map_while(Result::ok).collect();
    let input = parse(&input);

    one(&input);
    two(&input);
}
