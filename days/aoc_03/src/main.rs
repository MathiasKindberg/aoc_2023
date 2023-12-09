//! One: 525911
//! Two: 75805607

use std::{io::BufRead, vec};

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

fn find_adjacent_symbol(
    x: usize,
    y: usize,
    input: &[Vec<char>],
    symbol_check: fn(&char) -> bool,
) -> Option<(usize, usize)> {
    for (y, row) in input.iter().enumerate().skip(y - 1).take(3) {
        for (x, symbol) in row.iter().enumerate().skip(x - 1).take(3) {
            if symbol_check(symbol) {
                return Some((x, y));
            }
        }
    }
    None
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
                if !has_adjacent_symbol
                    && find_adjacent_symbol(x + PADDING, y + PADDING, &input, |symbol: &char| {
                        {
                            !symbol.is_ascii_digit() && symbol != &'.'
                        }
                    })
                    .is_some()
                {
                    has_adjacent_symbol = true
                }
                num = num * 10 + u64::from(char.to_digit(10).unwrap());
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

fn two(input: &[Vec<char>]) {
    const PADDING: usize = 1;

    let now = std::time::Instant::now();
    let mut gears = vec![vec![vec![]; input[0].len()]; input.len()];

    let input = pad_input(input.to_owned());

    for (y, row) in input[1..(input.len() - 1)].iter().enumerate() {
        let mut num: u64 = 0;
        let _has_adjacent_gear = false;
        let mut adjacent_gear = None;

        for (x, char) in row[1..(row.len())].iter().enumerate() {
            if char.is_ascii_digit() {
                if let Some((x, y)) =
                    find_adjacent_symbol(x + PADDING, y + PADDING, &input, |symbol: &char| {
                        symbol == &'*'
                    })
                {
                    adjacent_gear = Some((x, y));
                }
                num = num * 10 + u64::from(char.to_digit(10).unwrap());
            } else {
                if let Some((x, y)) = adjacent_gear {
                    gears[y][x].push(num)
                }

                adjacent_gear = None;
                num = 0;
            }
        }
    }

    let sum: u64 = gears
        .iter()
        .flat_map(|gears| {
            gears.iter().filter(|gear| gear.len() == 2).map(|gears| {
                gears
                    .iter()
                    .fold(None, |acc, val| match acc {
                        Some(acc) => Some(acc * val),
                        None => Some(*val),
                    })
                    .unwrap()
            })
        })
        .sum();

    println!("Two: {sum} | Elapsed: {:?}", now.elapsed());
}

fn main() {
    let input = input();

    one(&input);
    two(&input);
}
