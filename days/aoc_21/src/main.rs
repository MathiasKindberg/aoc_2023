//! Part 1:
//! Part 2:

use std::io::BufRead;

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
    let now = std::time::Instant::now();
    let sum = 0;

    aoc_lib::print_2d(&input);

    println!("One: {sum} | Elapsed: {:?}", now.elapsed());
}
fn two(_input: &Input) {
    let now = std::time::Instant::now();
    let sum = 0;

    println!("Two: {sum} | Elapsed: {:?}", now.elapsed());
}

fn parse(input: &Vec<String>) -> Input {
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
    let input = stdin.lock().lines().map_while(Result::ok).collect();

    let input = parse(&input);
    one(&input);
    two(&input);
}
