//! Part 1:
//! Part 2:

use std::io::BufRead;

fn input() -> Vec<Vec<char>> {
    let stdin = std::io::stdin();
    stdin
        .lock()
        .lines()
        .map_while(Result::ok)
        .map(|row| row.chars().collect::<Vec<_>>())
        .collect()
}

#[derive(Debug)]
struct Node {
    id: usize,
    symbol: char,

    // Connections:
    north: Option<usize>,
    south: Option<usize>,
    west: Option<usize>,
    east: Option<usize>,
}

impl Node {
    fn new(id: usize, symbol: char) -> Self {
        Self {
            id,
            symbol,
            north: None,
            south: None,
            west: None,
            east: None,
        }
    }
}

enum Direction {
    NorthSouth,
    EastWest,
}

fn check_compatability(a: char, b: char, direction: Direction) -> bool {
    match direction {
        Direction::NorthSouth => north_south(a, b),
        Direction::EastWest => east_west(a, b),
    }
}

fn north_south(a: char, b: char) -> bool {
    match (a, b) {
        // Ground with anything
        (a, '.') => false,
        ('.', b) => false,

        (a, b) => unreachable!("Unknown connection type"),
    }
}

fn east_west(a: char, b: char) -> bool {
    false
}

fn one(input: &[Vec<char>]) {
    let now = std::time::Instant::now();
    let sum = 0;
    let row_len = input[0].len();
    let start_node = 0;
    let mut map = Vec::with_capacity(input.len() * row_len);

    // Build map so we can reference all other nodes while creating the connections.
    for row in input {
        for c in row {
            map.push(Node::new(map.len() + 1, *c));
        }
    }

    // Build connections
    for (row_idx, row) in input.iter().enumerate() {
        let row_idx = row_idx * row_len;
        for (char_idx, c) in row.iter().enumerate() {
            let idx = row_idx + char_idx;
            // match c {
            //     // Do nothing
            //     &'.' | 'S' => (),

            //     // Connect North <-> South
            //     &'|' => {
            //         let north = map[idx - row_len];
            //         let south = map[idx + row_len];
            //     }

            //     err => unreachable!("Unknown charachter: `{err}`"),
            // }
        }
    }

    println!("{:?}", map[7]);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn north_south_connections() {
        assert!(!north_south('.', '|'))
    }
}
