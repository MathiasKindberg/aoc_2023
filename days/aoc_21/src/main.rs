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
    // aoc_lib::print_2d(&input);
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

fn two(input: &Input) {
    // aoc_lib::print_2d(&input);
    let now = std::time::Instant::now();

    let mut input = input.clone();
    let num_rows = input.len() as isize;
    let num_cols = input[0].len() as isize;

    let starting_position = input
        .iter()
        .enumerate()
        .find_map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .find(|(_, &tile)| tile == Tile::StartingPosition)
                .map(|(col_idx, _)| (col_idx as isize, row_idx as isize))
        })
        .unwrap();

    // Turn the starting position into a regular tile so we don't have to deal
    // with the special case when stepping around the grid.
    input[starting_position.0 as usize][starting_position.1 as usize] = Tile::GardenPlot;

    // All current positions.
    let mut positions: HashSet<(isize, isize)> = HashSet::from_iter(vec![starting_position]);

    // All possible next positions. Gets swapped into the positions when
    // a step is done.
    let mut next_positions: HashSet<(isize, isize)> = HashSet::new();

    let mut pos = vec![];

    let mut last_sum = 0;

    // To make this more efficient we could keep track of a seen list and know that
    // as soon as we step on to a tile on an odd number then it will always be seen
    // on next odd number. But yeah, 1.5s is fast enough.
    for step in 1..=65 + 131 * 2 {
        for (row, col) in positions.drain() {
            const POSSIBLE_STEPS: &[(isize, isize)] = &[
                (0, 1),  // Right
                (0, -1), // Left
                (-1, 0), // Up
                (1, 0),  // Down
            ];
            for (row_step, col_step) in POSSIBLE_STEPS {
                let row = row + row_step;
                let col = col + col_step;

                let mut map_row = row % num_rows;
                let mut map_col = col % num_cols;

                if map_row < 0 {
                    map_row += num_rows;
                }

                if map_col < 0 {
                    map_col += num_cols;
                }

                if input[map_row as usize][map_col as usize] == Tile::GardenPlot {
                    next_positions.insert((row, col));
                }
            }
        }

        assert!(positions.is_empty());
        positions.extend(next_positions.drain());
        assert!(next_positions.is_empty());
        if step == 65 || step % (1 * 131 + 65) == 0 || step % (2 * 131 + 65) == 0 {
            println!(
                "{step:>5} Positions: {:>6}: Diff: {:>6} | Elapsed: {:?}",
                positions.len(),
                positions.len() - last_sum,
                now.elapsed(),
            );
            // pos.push((step, positions.len()));
            pos.push(positions.len() as isize);

            last_sum = positions.len();
        }
    }

    // Geometric solution 100% based on Lagrange Interpolation found on reddit and this Quora reply
    // https://www.quora.com/How-do-you-find-the-equation-of-a-parabola-given-three-points
    println!("Interpolates from: {pos:?}");
    let x = 26501365 / 131;

    let x1 = 0;
    let x2 = 1;
    let x3 = 2;

    let y = ((x - x2) * (x - x3)) / ((x1 - x2) * (x1 - x3)) * pos[0]
        + ((x - x1) * (x - x3)) / ((x2 - x1) * (x2 - x3)) * pos[1]
        + ((x - x1) * (x - x2)) / ((x3 - x1) * (x3 - x2)) * pos[2];

    println!("Two: {y} | Elapsed: {:?}", now.elapsed());
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

    let res = 26501365 % 131;
    println!("Div by grid length: {res}");

    one(&input);
    two(&input);
}
