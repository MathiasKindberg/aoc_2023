/// Part 1: 55108
/// Part 2:
use std::io::BufRead;

fn input() -> Vec<Vec<u32>> {
    let stdin = std::io::stdin();
    stdin
        .lock()
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            line.chars()
                .filter_map(|digit| digit.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .collect()
}

fn one(input: &[Vec<u32>]) {
    let sum: u32 = input
        .iter()
        .filter(|line| !line.is_empty())
        // SAFETY: We filter empty lines and thus first and last must exist.
        .map(|line| line.last().unwrap() + (10 * line.first().unwrap()))
        .sum();
    println!("One: {sum}");
}

fn two(_input: &[Vec<u32>]) {}

fn main() {
    let input = input();
    one(&input);
    two(&input);
}
