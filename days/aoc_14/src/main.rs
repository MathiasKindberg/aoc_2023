//! Part 1: 110779
//! Part 2: 86069

use std::io::BufRead;

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
enum Ground {
    Round,
    Cube,
    Empty,
}

impl std::fmt::Display for Ground {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ground::Empty => write!(f, "."),
            Ground::Round => write!(f, "O"),
            Ground::Cube => write!(f, "#"),
        }
    }
}

fn input() -> Vec<Vec<Ground>> {
    let stdin = std::io::stdin();
    stdin
        .lock()
        .lines()
        .map_while(Result::ok)
        .map(|row| {
            row.chars()
                .map(|c| match c {
                    'O' => Ground::Round,
                    '#' => Ground::Cube,
                    '.' => Ground::Empty,
                    err => unreachable!("Unknown input: {err:?}"),
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn tilt_west(input: &mut Vec<Vec<Ground>>) {
    for row in input {
        // Compact row
        // Iterate by char, if empty grab next movable char and move next to it.
        // Could make it more efficient by moving groups of chars.
        for idx in 0..row.len() {
            match row[idx] {
                // Find next empty and move it here
                Ground::Empty => {
                    for movable_idx in idx + 1..row.len() {
                        let ground_type = &row[movable_idx];
                        match ground_type {
                            // Can't move a cube
                            Ground::Cube => break,
                            Ground::Round => {
                                row[idx] = *ground_type;
                                row[movable_idx] = Ground::Empty;

                                // Ensure that we do not try to move multiple rounds to the same location
                                break;
                            }
                            Ground::Empty => (),
                        }
                    }
                }
                // Do nothing
                Ground::Round | Ground::Cube => (),
            }
        }
    }
}

fn one(input: Vec<Vec<Ground>>) {
    let now = std::time::Instant::now();

    // Transpose input so we do not need to juggle columns
    let mut input = aoc_lib::transpose2(input);
    tilt_west(&mut input);
    let input = aoc_lib::transpose2(input);

    // Calculate load
    let total_load: usize = input
        .iter()
        .zip((1..input.len() + 1).rev())
        .map(|(row, load)| {
            row.iter()
                .filter(|ground| ground == &&Ground::Round)
                .count()
                * load
        })
        .sum();

    println!("One: {total_load} | Elapsed: {:?}", now.elapsed());
}

fn cycle(input: &mut Vec<Vec<Ground>>) {
    for _ in 0..4 {
        aoc_lib::rotate_90_cw_2d(input);
        tilt_west(input);
    }
}

fn two(mut input: Vec<Vec<Ground>>) {
    const CYCLES: usize = 1_000_000_000;
    let now = std::time::Instant::now();

    // -180 degrees means north -> south. Then we rotate clock-wise means what originally
    // was north -> west. Then we tilt west and start cycling.
    aoc_lib::rotate_90_ccw_2d(&mut input);
    aoc_lib::rotate_90_ccw_2d(&mut input);

    let mut seen: Vec<_> = vec![input.clone()];
    let mut final_grid = None;

    for _ in 0..CYCLES {
        cycle(&mut input);

        if let Some(idx) = seen.iter().position(|x| x == &input) {
            let cycle_len = seen.len() - idx;
            let final_idx = idx + (CYCLES - idx) % cycle_len;
            final_grid = Some(seen[final_idx].clone());
            break;
        }

        seen.push(input.clone());
    }

    input = final_grid.unwrap();

    // Undo our initial rotation.
    aoc_lib::rotate_90_ccw_2d(&mut input);
    aoc_lib::rotate_90_ccw_2d(&mut input);

    let total_load: usize = input
        .iter()
        .zip((1..input.len() + 1).rev())
        .map(|(row, load)| {
            row.iter()
                .filter(|ground| ground == &&Ground::Round)
                .count()
                * load
        })
        .sum();

    println!("Two: {total_load} | Elapsed: {:?}", now.elapsed());
}

fn main() {
    let input = input();
    one(input.clone());
    two(input);
}
