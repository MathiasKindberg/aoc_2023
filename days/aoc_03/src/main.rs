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
    for y in (y - 1)..=(y + 1) {
        for x in (x - 1)..=(x + 1) {
            let symbol = &input[y][x];
            if !symbol.is_ascii_digit() && symbol != &'.' {
                return true;
            }
        }
    }
    false
}

#[derive(Debug)]
enum Part {
    Y(u64),
    N(u64),
}

fn one(input: &[Vec<char>]) {
    const PADDING: usize = 1;

    let now = std::time::Instant::now();
    let input = pad_input(input.to_owned());
    let mut sum = 0;

    for (y, row) in (&input[1..(input.len() - 1)]).iter().enumerate() {
        let mut num: u64 = 0;
        let mut has_adjacent_symbol = false;
        let mut found_nums = vec![];
        for (x, char) in (&row[1..(row.len() - 1)]).iter().enumerate() {
            if char.is_ascii_digit() {
                if adjacent_symbol(x + PADDING, y + PADDING, &input) {
                    has_adjacent_symbol = true
                }
                let digit: u64 = char.to_digit(10).unwrap().into();
                num = num * 10 + digit;
            } else {
                if has_adjacent_symbol {
                    found_nums.push(Part::Y(num));
                    sum += num;
                } else {
                    if num != 0 {
                        found_nums.push(Part::N(num));
                    }
                }
                has_adjacent_symbol = false;
                num = 0;
            }
        }
        println!("{found_nums:?}");
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
