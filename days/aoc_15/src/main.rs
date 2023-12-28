//! Part 1: 515495
//! Part 2:

use std::io::BufRead;

fn input() -> Vec<String> {
    let stdin = std::io::stdin();
    stdin.lock().lines().map_while(Result::ok).collect()
}

fn hash(mut current_value: u64, c: char) -> u64 {
    current_value += c as u64;
    current_value *= 17;
    current_value %= 256;
    current_value
}

fn encode(s: &str) -> u64 {
    s.chars().fold(0, hash)
}

fn one(input: &[String]) {
    let now = std::time::Instant::now();
    let sum: u64 = input[0].split(',').map(encode).sum();
    println!("One: {sum} | Elapsed: {:?}", now.elapsed());
}

#[derive(Debug, Clone)]
struct Lens<'a> {
    label: &'a str,
    focal_length: usize,
    hash: u64,
}

fn two(input: &[String]) {
    let now = std::time::Instant::now();
    let sum = 0;
    let operations: Vec<_> = input[0].split(',').collect();

    // Start with 255 empty boxes to ensure that all boxes we index into
    // based on the "HASH" exists.
    let boxes: Vec<Vec<Lens>> = vec![vec![]; 255];
    println!("{operations:?}");
    for op in operations {
        if op.contains('=') {
            let mut op = op.split('=');
            let label = op.next().unwrap();
            let lens = Lens {
                label,
                focal_length: op.next().unwrap().parse::<usize>().unwrap(),
                hash: encode(label),
            };

            // dbg!(encode(lens.label));
            println!("{lens:?}");
            // if let Some(idx) = boxes
            //     .iter()
            //     .position(|lens_box| lens_box.label == lens.label)
            // {
            // } else {
            // }
        } else if op.contains('-') {
            let label = op.replace('-', "");
            println!("{label:5} -> --")
        } else {
            unreachable!("Unknown symbol in {op}")
        }
    }

    println!("Two: {sum} | Elapsed: {:?}", now.elapsed());
}

fn main() {
    let input = input();
    assert_eq!(
        input.len(),
        1,
        "Ignoring new lines means the input is one line"
    );
    one(&input);
    two(&input);
}
