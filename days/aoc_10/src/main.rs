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
    id: (usize, usize),
    symbol: char,

    connections: Vec<(usize, usize)>,
    // // Connections:
    // north: Option<(usize, usize)>,
    // south: Option<(usize, usize)>,
    // west: Option<(usize, usize)>,
    // east: Option<(usize, usize)>,
}

impl Node {
    fn new(id: (usize, usize), symbol: char) -> Self {
        Self {
            id,
            symbol,
            connections: Vec::new(),
        }
    }

    fn add_connection(&mut self, id: (usize, usize)) {
        self.connections.push(id)
    }
}

fn north_south(northern: char, southern: char) -> bool {
    // We only need to specify the valid directions
    match (northern, southern) {
        // Straights
        ('|', '|') => true,

        // Curves
        // Up -> Down -> Right/Left
        ('|', 'L') => true,
        ('|', 'J') => true,

        // Down -> Up -> Right/Left
        ('7', '|') => true,
        ('F', '|') => true,

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

fn west_east(western: char, eastern: char) -> bool {
    // We only need to specify the valid directions
    match (western, eastern) {
        // Straights
        ('-', '-') => true,

        // Curves
        // Right -> Up/Down
        ('-', '7') => true,
        ('-', 'J') => true,

        // Down -> Up -> Right/Left
        ('L', '-') => true,
        ('F', '-') => true,

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
    let sum = 0;
    let input = pad_input(input.to_vec());

    // TODO: We can skip building the map and instead find connections as we go....
    let mut map: Vec<Vec<Node>> = Vec::with_capacity(input.len());
    // Build map so we can reference all other nodes while creating the connections.
    for (row_idx, row) in input.iter().enumerate() {
        map.push(
            row.iter()
                .enumerate()
                .map(|(char_idx, c)| Node::new((row_idx, char_idx), *c))
                .collect(),
        );
    }

    // Build connections.
    // TODO We could do this together with initializing the map since we only check up and left. But whatever.
    let mut starting_position = None;
    for row_idx in 1..(map.len() - 1) {
        for char_idx in 1..(map[row_idx].len() - 1) {
            if map[row_idx][char_idx].symbol == 'S' {
                starting_position = Some((row_idx, char_idx));
            }
            print!("{}", &map[row_idx][char_idx].symbol);
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
        println!("")
    }

    let mut position = starting_position.expect("one to exist");
    let mut steps = 0;
    loop {}
    dbg!(&starting_position);
    // Depth first search

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
    use pretty_assertions::assert_eq;

    #[test]
    fn north_south_connections() {
        let test_table = [
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
}
