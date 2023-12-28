//! Part 1: 515495
//! Part 2:

use std::io::BufRead;

fn input() -> Vec<String> {
    let stdin = std::io::stdin();
    stdin.lock().lines().map_while(Result::ok).collect()
}

fn hash(mut current_value: usize, c: char) -> usize {
    current_value += c as usize;
    current_value *= 17;
    current_value %= 256;
    current_value
}

fn encode(s: &str) -> usize {
    s.chars().fold(0, hash)
}

fn one(input: &[String]) {
    let now = std::time::Instant::now();
    let sum: usize = input[0].split(',').map(encode).sum();
    println!("One: {sum} | Elapsed: {:?}", now.elapsed());
}

#[derive(Debug, Clone)]
struct Lens<'a> {
    label: &'a str,
    focal_length: usize,
    // Could maybe make sense to instead have a new and return
    // (Self, u64)
    hash: usize,
}

fn two(input: &[String]) {
    let now = std::time::Instant::now();
    let sum = 0;
    let operations: Vec<_> = input[0].split(',').collect();

    // Start with 255 empty boxes to ensure that all boxes we index into
    // based on the "HASH" exists.
    let mut boxes: Vec<Vec<Lens>> = vec![vec![]; 256];
    println!("{operations:?}");
    for op in operations {
        if op.contains('=') {
            let mut op = op.split('=');

            // Before '=' is the label
            let label = op.next().unwrap();
            let lens = Lens {
                label,
                // After '=' is the focal length
                focal_length: op.next().unwrap().parse::<usize>().unwrap(),
                hash: encode(label),
            };

            let lens_box = &mut boxes[lens.hash];
            if let Some(existing_lens) = lens_box.iter_mut().find(|item| item.label == lens.label) {
                existing_lens.focal_length = lens.focal_length;
            } else {
                lens_box.push(lens);
            }
        } else if op.contains('-') {
            let label = &op[0..op.len() - 1];
            let hash = encode(&op[0..op.len() - 1]);
            let lens_box = &mut boxes[hash];
            lens_box.retain_mut(|elem| elem.label != label);
        } else {
            unreachable!("Unknown symbol in {op}")
        }
    }

    for (idx, boxy) in boxes.iter().enumerate() {
        if !boxy.is_empty() {
            println!("{idx}: {boxy:?}");
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
