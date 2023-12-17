//! Part 1: 6786
//! Part 2:

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

    connections: Vec<(usize, usize)>,
}

impl Node {
    fn new(symbol: char) -> Self {
        Self {
            symbol,
            connections: Vec::new(),
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

    // TODO: We can skip building the map and instead find connections as we go....
    let mut map: Vec<Vec<Node>> = Vec::with_capacity(input.len());
    // Build map so we can reference all other nodes while creating the connections.
    for row in input.iter() {
        map.push(row.iter().map(|c| Node::new(*c)).collect());
    }

    // Build connections.
    // TODO We could do this together with initializing the map since we only check up and left. But whatever.
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
