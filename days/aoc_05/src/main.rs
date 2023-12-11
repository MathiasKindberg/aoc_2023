//! Part 1: 993500720
//! Part 2: 4917124, brute force in 3 minutes.
//!
//!
//! Part 2 smart solutions:
//! 1. Reverse search.
//! 2. Treat the seeds as ranges and split them as necessary.
//!

use std::{io::BufRead, vec};

fn input() -> Vec<String> {
    let stdin = std::io::stdin();
    stdin.lock().lines().map_while(Result::ok).collect()
}

enum Type<'a> {
    Name(&'a String),
    Mapping((u64, u64, u64)),
}

fn one(input: &[String]) {
    use itertools::Itertools;

    let now = std::time::Instant::now();

    let mut seeds: Vec<_> = input[0]
        .trim_start_matches("seeds: ")
        .split(' ')
        .map(|seed_number| seed_number.parse::<u64>().unwrap())
        .collect();

    let input: &Vec<Type> = &input[2..]
        .iter()
        .filter(|row| !row.is_empty())
        .map(|row| {
            if row.contains("map") {
                Type::Name(row)
            } else {
                Type::Mapping(
                    row.split_ascii_whitespace()
                        .map(|num| num.parse::<u64>().unwrap())
                        .collect_tuple()
                        .unwrap(),
                )
            }
        })
        .collect();

    let mut moved = vec![false; seeds.len()];
    for row in input {
        match row {
            Type::Name(_) => {
                for v in &mut moved {
                    *v = false;
                }
            }
            Type::Mapping((destination, source, range)) => {
                for (idx, seed) in seeds.iter_mut().enumerate() {
                    if *seed >= *source && *seed <= source + (range - 1) && !moved[idx] {
                        *seed = destination + (*seed - source);
                        moved[idx] = true;
                    }
                }
            }
        }
    }

    let min = seeds.iter().min().unwrap();
    println!("One: {min} | Elapsed: {:?}", now.elapsed());
}

fn two_stupid_version(input: &[String]) {
    use itertools::Itertools;

    let now = std::time::Instant::now();

    let seed_ranges: Vec<(u64, u64)> = input[0]
        .trim_start_matches("seeds: ")
        .split(' ')
        .map(|seed_number| seed_number.parse::<u64>().unwrap())
        .tuples()
        .map(|(start, length)| (start, length))
        .collect();

    println!("Ranges: {seed_ranges:?}");
    let mut seeds: Vec<u64> = seed_ranges
        .into_iter()
        .flat_map(|(from, range)| (from..(from + range)).collect::<Vec<_>>())
        .collect();
    println!("Split into seeds");
    let input: &Vec<Type> = &input[2..]
        .iter()
        .filter(|row| !row.is_empty())
        .map(|row| {
            if row.contains("map") {
                Type::Name(row)
            } else {
                Type::Mapping(
                    row.split_ascii_whitespace()
                        .map(|num| num.parse::<u64>().unwrap())
                        .collect_tuple()
                        .unwrap(),
                )
            }
        })
        .collect();

    println!("Starting mapping");
    let mut moved = vec![false; seeds.len()];
    for row in input {
        match row {
            Type::Name(name) => {
                println!("Starting: {name}");
                for v in &mut moved {
                    *v = false;
                }
            }
            Type::Mapping((destination, source, range)) => {
                for (idx, seed) in seeds.iter_mut().enumerate() {
                    if *seed >= *source && *seed <= source + (range - 1) && !moved[idx] {
                        *seed = destination + (*seed - source);
                        moved[idx] = true;
                    }
                }
            }
        }
    }

    let min = seeds.iter().min().unwrap();
    println!("Two: {min} | Elapsed: {:?}", now.elapsed());
}

fn main() {
    let input = input();
    one(&input);
    two_stupid_version(&input);
}
