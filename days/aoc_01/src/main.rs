//! Part 1: 55108
//! Part 2: 56324

use std::collections::HashMap;
use std::io::BufRead;
use std::sync::OnceLock;

fn input() -> Vec<String> {
    let stdin = std::io::stdin();
    stdin.lock().lines().map_while(Result::ok).collect()
}

fn one(input: &[String]) {
    let now = std::time::Instant::now();
    let sum: u32 = input
        .iter()
        .map(|line| {
            line.chars()
                .filter_map(|digit| digit.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .filter(|line| !line.is_empty())
        // SAFETY: We filter empty lines and thus first and last must exist.
        .map(|line| line.last().unwrap() + (10 * line.first().unwrap()))
        .sum();
    println!("One: {sum} | Elapsed: {:?}", now.elapsed());
}

fn lookup_table() -> &'static HashMap<&'static str, u64> {
    static HASHMAP: OnceLock<HashMap<&str, u64>> = OnceLock::new();

    HASHMAP.get_or_init(|| HashMap::from_iter(LOOKUP_TABLE.iter().cloned()))
}

const LOOKUP_TABLE: &[(&str, u64)] = &[
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn get_digit(row: &str) -> Option<u64> {
    let mut found_digits: Vec<_> = lookup_table()
        .iter()
        .flat_map(|(num, _)| row.match_indices(num))
        .collect();

    found_digits.sort_by(|(idx_a, _), (idx_b, _)| idx_a.cmp(idx_b));

    Some(
        lookup_table().get(found_digits.last()?.1)?
            + (10 * lookup_table().get(found_digits.first()?.1)?),
    )
}

fn two(input: &[String]) {
    let now = std::time::Instant::now();
    let sum: u64 = input.iter().filter_map(|row| get_digit(row)).sum();
    println!("Two: {sum} | Elapsed: {:?}", now.elapsed());
}

fn main() {
    let input = input();
    one(&input);
    two(&input);
}
