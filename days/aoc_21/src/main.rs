//! Part 1:
//! Part 2:

use std::{collections::HashSet, io::BufRead, vec};

type Input = Vec<Vec<Tile>>;

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
enum Tile {
    StartingPosition,
    Rock,
    GardenPlot,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::GardenPlot => write!(f, "·"), // Switch to interpunct "."
            Tile::StartingPosition => write!(f, "S"),
            Tile::Rock => write!(f, "#"),
        }
    }
}

fn one(input: &Input) {
    // Pad input so we don't have to deal with checking walls when indexing
    let mut input = aoc_lib::pad_input(input.clone(), Tile::Rock);
    aoc_lib::print_2d(&input);
    let now = std::time::Instant::now();

    let starting_position = input
        .iter()
        .enumerate()
        .find_map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .find(|(_, &tile)| tile == Tile::StartingPosition)
                .map(|(col_idx, _)| (col_idx, row_idx))
        })
        .unwrap();

    // Turn the starting position into a regular tile so we don't have to deal
    // with the special case when stepping around the grid.
    input[starting_position.0][starting_position.1] = Tile::GardenPlot;

    // All current positions.
    let mut positions: HashSet<(usize, usize)> = HashSet::from_iter(vec![starting_position]);

    // All possible next positions. Gets swapped into the positions when
    // a step is done.
    let mut next_positions: HashSet<(usize, usize)> = HashSet::new();

    for _ in 1..=64 {
        for (row, col) in positions.drain() {
            const POSSIBLE_STEPS: &[(isize, isize)] = &[
                (0, 1),  // Right
                (0, -1), // Left
                (-1, 0), // Up
                (1, 0),  // Down
            ];
            for (row_step, col_step) in POSSIBLE_STEPS {
                let row = (row as isize + row_step) as usize;
                let col = (col as isize + col_step) as usize;
                if input[row][col] == Tile::GardenPlot {
                    next_positions.insert((row, col));
                }
            }
        }

        assert!(positions.is_empty());
        positions.extend(next_positions.drain());
        assert!(next_positions.is_empty());
    }

    let sum = positions.len();
    println!("One: {sum} | Elapsed: {:?}", now.elapsed());
}
fn two(_input: &Input) {
    let now = std::time::Instant::now();
    let sum = 0;

    println!("Two: {sum} | Elapsed: {:?}", now.elapsed());
}

fn parse(input: &[String]) -> Input {
    input
        .iter()
        .map(|row| {
            row.chars()
                .map(|c| match c {
                    'S' => Tile::StartingPosition,
                    '#' => Tile::Rock,
                    '.' => Tile::GardenPlot,
                    err => unreachable!("Unknown input: {err:?}"),
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn main() {
    let stdin = std::io::stdin();
    let input: Vec<_> = stdin.lock().lines().map_while(Result::ok).collect();

    let input = parse(&input);
    one(&input);
    two(&input);
}
