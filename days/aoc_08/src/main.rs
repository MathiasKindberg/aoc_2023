//! Part 1:
//! Part 2:

use std::{collections::HashMap, io::BufRead};

fn input() -> Vec<String> {
    let stdin = std::io::stdin();
    stdin.lock().lines().map_while(Result::ok).collect()
}

#[derive(Debug)]
struct Destination {
    left: String,
    right: String,
}

fn one(input: &[String]) {
    use itertools::Itertools;

    let now = std::time::Instant::now();
    let sum = 0;

    println!("{input:?}");

    // let movement = input[0].

    let input: HashMap<_, _> = input[2..]
        .iter()
        .map(|row| row.split(" = ").collect_tuple().unwrap())
        .map(|(source, destination)| {
            let cleaned_input = destination
                .replace('(', "")
                .replace(')', "")
                .replace(",", "");
            let (left, right) = cleaned_input
                .split_ascii_whitespace()
                .collect_tuple()
                .unwrap();
            (
                source.to_owned(),
                Destination {
                    left: left.to_owned(),
                    right: right.to_owned(),
                },
            )
        })
        .collect();

    for row in input {
        println!("{row:?}");
    }

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
