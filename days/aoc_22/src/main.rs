//! Part 1:
//! Part 2:

use std::{io::BufRead, str::FromStr};

type Input = Vec<Brick>;

/// Expects all positions to be positive which aligns with example
/// and input.
#[derive(Debug)]
struct Coord {
    x: u64,
    y: u64,
    z: u64,
}

impl FromStr for Coord {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut positions = s.split(',');
        Ok(Self {
            x: positions.next().unwrap().parse().unwrap(),
            y: positions.next().unwrap().parse().unwrap(),
            z: positions.next().unwrap().parse().unwrap(),
        })
    }
}

#[derive(Debug)]
struct Brick {
    from: Coord,
    to: Coord,
}

/// Solving:
/// 1. Create 3d map of all bricks in space.
/// 2. Step Z by Z and bring bricks to ground
/// 3. Eliminate bricks with more than 1 contact point.
fn one(input: &Input) {
    let now = std::time::Instant::now();
    let sum = 0;

    for row in input {
        println!("{row:?}");
    }

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
            let (p1, p2) = row.split_once("~").unwrap();
            Brick {
                from: Coord::from_str(p1).unwrap(),
                to: Coord::from_str(p2).unwrap(),
            }
        })
        .collect()
}

fn main() {
    // Keep input owned by main function to allow nifty references.
    let stdin = std::io::stdin();
    let input: Vec<String> = stdin.lock().lines().map_while(Result::ok).collect();
    let input = parse(&input);

    one(&input);
    two(&input);
}
