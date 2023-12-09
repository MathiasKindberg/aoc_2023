//! One: 525911
//! Two:

use std::io::BufRead;

fn input() -> Vec<Vec<char>> {
    let stdin = std::io::stdin();
    stdin
        .lock()
        .lines()
        .map_while(Result::ok)
        .map(|row| row.chars().collect())
        .collect()
}

/// Adds a padding layer of dots around the schematic ensuring
/// we do not have to deal with the edges.
fn pad_input(mut input: Vec<Vec<char>>) -> Vec<Vec<char>> {
    assert!(!input.is_empty(), "Expected input");

    for row in input.iter_mut() {
        row.insert(0, '.');
        row.push('.')
    }

    let top_bottom_padding: Vec<_> = ".".repeat(input[0].len()).chars().collect();
    input.insert(0, top_bottom_padding.clone());
    input.push(top_bottom_padding);

    input
}

/// `...`
/// `.X.`
/// `...`
fn adjacent_symbol(x: usize, y: usize, input: &[Vec<char>]) -> bool {
    for row in input.iter().skip(y - 1).take(3) {
        for symbol in row.iter().skip(x - 1).take(3) {
            if !symbol.is_ascii_digit() && symbol != &'.' {
                return true;
            }
        }
    }
    false
}

fn one(input: &[Vec<char>]) {
    const PADDING: usize = 1;

    let now = std::time::Instant::now();
    let input = pad_input(input.to_owned());
    let mut sum = 0;

    for (y, row) in input[1..(input.len() - 1)].iter().enumerate() {
        let mut num: u64 = 0;
        let mut has_adjacent_symbol = false;

        for (x, char) in row[1..(row.len())].iter().enumerate() {
            if char.is_ascii_digit() {
                if !has_adjacent_symbol && adjacent_symbol(x + PADDING, y + PADDING, &input) {
                    has_adjacent_symbol = true
                }
                let digit: u64 = char.to_digit(10).unwrap().into();
                num = num * 10 + digit;
            } else {
                if has_adjacent_symbol {
                    sum += num;
                }

                has_adjacent_symbol = false;
                num = 0;
            }
        }
    }

    println!("One: {sum} | Elapsed: {:?}", now.elapsed());
}
fn two(_input: &[Vec<char>]) {
    let now = std::time::Instant::now();
    let sum = 0;

    println!("Two: {sum} | Elapsed: {:?}", now.elapsed());
}

fn main() {
    let input = input();

    one(&input);
    two(&input);
}
