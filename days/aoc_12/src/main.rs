//! Part 1:
//! Part 2:

use std::io::BufRead;

fn input() -> Vec<String> {
    let stdin = std::io::stdin();
    stdin.lock().lines().map_while(Result::ok).collect()
}

fn one(input: &[String]) {
    use itertools::Itertools;
    let now = std::time::Instant::now();
    let sum = 0;
    let input: Vec<(&str, Vec<usize>)> = input
        .iter()
        .map(|row| row.split_ascii_whitespace().collect_tuple().unwrap())
        .map(|(groups, numbers)| {
            (
                groups,
                numbers
                    .split(',')
                    .map(|num| num.parse::<usize>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .collect();

    println!("{input:#?}");

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
