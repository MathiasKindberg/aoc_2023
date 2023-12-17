//! Part 1: 6786
//! Part 2:

use core::num;
use std::{collections::HashSet, io::BufRead};

fn input() -> Vec<Vec<char>> {
    let stdin = std::io::stdin();
    stdin
        .lock()
        .lines()
        .map_while(Result::ok)
        .map(|row| row.chars().collect::<Vec<_>>())
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

#[derive(Debug)]
struct Node {
    symbol: char,
    main_loop: bool,

    connections: Vec<(usize, usize)>,
}

impl Node {
    fn new(symbol: char) -> Self {
        Self {
            symbol,
            connections: Vec::new(),
            main_loop: false,
        }
    }

    fn add_connection(&mut self, id: (usize, usize)) {
        self.connections.push(id)
    }
}

fn north_south(north: char, south: char) -> bool {
    // We only need to specify the valid directions
    match (north, south) {
        // Straights
        ('|', '|') => true,

        // Curves
        // Up -> Down -> Right/Left
        ('|', 'L') => true,
        ('|', 'J') => true,

        // Down -> Up -> Right/Left
        ('7', '|') => true,
        ('F', '|') => true,

        // Curve to curve
        ('7', 'J') => true,
        ('7', 'L') => true,

        ('F', 'J') => true,
        ('F', 'L') => true,

        // Starting position
        ('S', '|') => true,
        ('|', 'S') => true,

        ('S', 'L') => true,
        ('S', 'J') => true,
        ('7', 'S') => true,
        ('F', 'S') => true,

        // Rest
        (_, _) => false,
    }
}

fn west_east(west: char, east: char) -> bool {
    // We only need to specify the valid directions
    match (west, east) {
        // Straights
        ('-', '-') => true,

        // Curves
        // Right -> Up/Down
        ('-', '7') => true,
        ('-', 'J') => true,

        // Down -> Up -> Right/Left
        ('L', '-') => true,
        ('F', '-') => true,

        // Curves to curves
        ('L', 'J') => true,
        ('L', '7') => true,

        ('F', 'J') => true,
        ('F', '7') => true,

        // Starting position
        ('S', '-') => true,
        ('-', 'S') => true,

        ('S', 'J') => true,
        ('S', '7') => true,
        ('L', 'S') => true,
        ('F', 'S') => true,

        // Rest
        (_, _) => false,
    }
}

fn one(input: &[Vec<char>]) {
    const PADDING: usize = 1;

    let now = std::time::Instant::now();
    let input = pad_input(input.to_vec());

    let mut map: Vec<Vec<Node>> = Vec::with_capacity(input.len());
    // Build map so we can reference all other nodes while creating the connections.
    for row in input.iter() {
        map.push(row.iter().map(|c| Node::new(*c)).collect());
    }

    // Build connections.
    let mut starting_position = None;
    for row_idx in PADDING..(map.len() - PADDING) {
        for char_idx in PADDING..(map[row_idx].len() - PADDING) {
            if map[row_idx][char_idx].symbol == 'S' {
                starting_position = Some((row_idx, char_idx));
            }
            // print!("{}", &map[row_idx][char_idx].symbol);
            if north_south(
                map[row_idx - 1][char_idx].symbol,
                map[row_idx][char_idx].symbol,
            ) {
                map[row_idx - 1][char_idx].add_connection((row_idx, char_idx));
                map[row_idx][char_idx].add_connection((row_idx - 1, char_idx));
            }

            if west_east(
                map[row_idx][char_idx - 1].symbol,
                map[row_idx][char_idx].symbol,
            ) {
                map[row_idx][char_idx - 1].add_connection((row_idx, char_idx));
                map[row_idx][char_idx].add_connection((row_idx, char_idx - 1));
            }
        }
    }

    // Depth first search. Hacky step counting which works since it never branches.
    let starting_position = starting_position.expect("one to exist");
    let mut stack = vec![starting_position];
    let mut discovered = HashSet::new();
    let mut steps: usize = 0;

    while let Some((row_idx, char_idx)) = stack.pop() {
        if discovered.insert((row_idx, char_idx)) {
            steps += 1;

            for (edge_row_idx, edge_char_idx) in &map[row_idx][char_idx].connections {
                stack.push((*edge_row_idx, *edge_char_idx));
            }
        }
    }

    let farthest = steps / 2;

    println!("One: {farthest} | Elapsed: {:?}", now.elapsed());
}

fn two(input: &[Vec<char>]) {
    const PADDING: usize = 1;

    let now = std::time::Instant::now();
    let input = pad_input(input.to_vec());

    let mut map: Vec<Vec<Node>> = Vec::with_capacity(input.len());
    // Build map so we can reference all other nodes while creating the connections.
    for row in input.iter() {
        map.push(row.iter().map(|c| Node::new(*c)).collect());
    }

    // Build connections.
    let mut starting_position = None;
    for row_idx in PADDING..(map.len() - PADDING) {
        for char_idx in PADDING..(map[row_idx].len() - PADDING) {
            if map[row_idx][char_idx].symbol == 'S' {
                starting_position = Some((row_idx, char_idx));
            }
            // print!("{}", &map[row_idx][char_idx].symbol);
            if north_south(
                map[row_idx - 1][char_idx].symbol,
                map[row_idx][char_idx].symbol,
            ) {
                map[row_idx - 1][char_idx].add_connection((row_idx, char_idx));
                map[row_idx][char_idx].add_connection((row_idx - 1, char_idx));
            }

            if west_east(
                map[row_idx][char_idx - 1].symbol,
                map[row_idx][char_idx].symbol,
            ) {
                map[row_idx][char_idx - 1].add_connection((row_idx, char_idx));
                map[row_idx][char_idx].add_connection((row_idx, char_idx - 1));
            }
        }
    }

    // Depth first search. Hacky step counting which works since it never branches.
    let starting_position = starting_position.expect("one to exist");
    let mut stack = vec![starting_position];
    let mut discovered = HashSet::new();

    while let Some((row_idx, char_idx)) = stack.pop() {
        if discovered.insert((row_idx, char_idx)) {
            map[row_idx][char_idx].main_loop = true;

            for (edge_row_idx, edge_char_idx) in &map[row_idx][char_idx].connections {
                stack.push((*edge_row_idx, *edge_char_idx));
            }
        }
    }

    const TEST: usize = 8;

    for (idx, row) in map.iter().enumerate() {
        for node in row {
            print!("{}", node.symbol);
        }
        println!("");
        if idx == TEST {
            break;
        }
    }

    println!("");

    // Scan each row to find inside and outside
    for (idx, row) in map.iter().enumerate() {
        let mut inside = false;
        let mut num_inside = 0;
        let mut last_node_main_loop = false;

        for node in row {
            let mut char = '0';

            if node.main_loop && !last_node_main_loop {
                inside = true
            } else if !node.main_loop && inside {
                inside = false
            }

            last_node_main_loop = node.main_loop;

            if inside && !node.main_loop {
                num_inside += 1;
            }

            // if node.main_loop {
            //     char = '#';
            // }

            // if !inside && node.main_loop {
            //     inside = true;
            // } else if !node.main_loop {
            //     inside = false;
            // }

            // if inside {
            //     char = '1';
            //     num_inside += 1
            // }
            // print!("{char}");
        }
        println!(": {num_inside}");
        if idx == TEST {
            break;
        }
    }
    println!("Two: asd | Elapsed: {:?}", now.elapsed());
}

fn main() {
    let input = input();
    one(&input);
    two(&input);
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn north_south_connections() {
        let test_table = [
            // Curves to Curves
            ('7', 'J', true),
            // Straights
            ('|', '|', true),
            ('|', '-', false),
            ('-', '|', false),
            ('-', '-', false),
            // To ground
            ('.', '|', false),
            ('|', '.', false),
        ];

        for (a, b, res) in test_table {
            assert_eq!(north_south(a, b), res);
        }
    }

    #[test]
    fn west_east_connections() {
        let test_table = [('L', 'J', true), ('L', '7', true), ('F', '7', true)];

        for (a, b, res) in test_table {
            assert_eq!(west_east(a, b), res);
        }
    }
}
