//! Part 1: 252000
//! Part 2:

use std::io::BufRead;

fn input() -> Vec<String> {
    let stdin = std::io::stdin();
    stdin.lock().lines().map_while(Result::ok).collect()
}

fn one(input: &[String]) {
    let now = std::time::Instant::now();

    let times = input[0]
        .trim_start_matches("Time:")
        .trim()
        .split_ascii_whitespace()
        .map(|val| val.parse::<u64>().unwrap());
    let distances = input[1]
        .trim_start_matches("Distance:")
        .trim()
        .split_ascii_whitespace()
        .map(|val| val.parse::<u64>().unwrap());

    let input: Vec<_> = times
        .zip(distances)
        // Adding +1 to distance since we need to go longer.
        // Casting to f64 for sqrt, and we expect numbers to be small enough to not lose precision.
        .map(|(time, distance)| (time as f64, (distance + 1) as f64))
        .collect();

    let res = input.iter().fold(0, |mut acc, (time, distance)| {
        let root = f64::sqrt((time.powi(2) / 4.0) - distance);
        let x1 = ((time / 2.0) - root).ceil();
        let x2 = ((time / 2.0) + root).floor();
        let num = (x2 - x1) + 1.0; // Add one to get how many numbers are in the range x1..=x2
        if acc == 0 {
            acc = num as u64
        } else {
            acc *= num as u64
        }
        acc
    });

    println!("One: {res} | Elapsed: {:?}", now.elapsed());
}

fn two(input: &[String]) {
    let now = std::time::Instant::now();

    let time = input[0]
        .trim_start_matches("Time:")
        .replace(' ', "")
        .parse::<u64>()
        .unwrap() as f64;

    let distance = input[1]
        .trim_start_matches("Distance:")
        .replace(' ', "")
        .parse::<u64>()
        .unwrap() as f64
        + 1.0;

    let root = f64::sqrt((time.powi(2) / 4.0) - distance);
    let x1 = ((time / 2.0) - root).ceil();
    let x2 = ((time / 2.0) + root).floor();
    let res = (x2 - x1) + 1.0; // Add one to get how many numbers are in the range x1..=x2

    println!("Two: {res} | Elapsed: {:?}", now.elapsed());
}

fn main() {
    let input = input();
    one(&input);
    two(&input);
}
