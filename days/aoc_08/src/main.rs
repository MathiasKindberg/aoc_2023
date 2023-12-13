//! Part 1: 11309
//! Part 2:

use std::{collections::HashMap, io::BufRead};

fn input() -> Vec<String> {
    let stdin = std::io::stdin();
    stdin.lock().lines().map_while(Result::ok).collect()
}

#[derive(Debug)]
struct Destination {
    left: String,
    right: String,
}

impl Destination {
    fn next(&self, movement: &char) -> &str {
        match movement {
            &'L' => &self.left,
            &'R' => &self.right,
            unknown => panic!("Unknown movement: {unknown}"),
        }
    }
}

fn one(input: &[String]) {
    use itertools::Itertools;

    const START_NODE: &str = "AAA";
    const FINISH_NODE: &str = "ZZZ";

    let now = std::time::Instant::now();
    let movement: Vec<_> = input[0].chars().collect();

    let map: HashMap<_, _> = input[2..]
        .iter()
        .map(|row| row.split(" = ").collect_tuple().unwrap())
        .map(|(source, destination)| {
            let cleaned_input = destination.replace(['(', ')', ','], "");
            let (left, right) = cleaned_input
                .split_ascii_whitespace()
                .collect_tuple()
                .unwrap();
            (
                source.to_owned(),
                Destination {
                    left: left.to_owned(),
                    right: right.to_owned(),
                },
            )
        })
        .collect();

    let mut steps = 0;
    let mut node = map.get(START_NODE).unwrap();
    for movement in movement.iter().cycle() {
        steps += 1;
        let next = node.next(movement);
        if next == FINISH_NODE {
            break;
        }
        node = map.get(next).unwrap();
    }

    println!("One: {steps} | Elapsed: {:?}", now.elapsed());
}
fn two(input: &[String]) {
    use itertools::Itertools;

    const START_NODE: &str = "AAA";
    const FINISH_NODE: &str = "ZZZ";

    let now = std::time::Instant::now();
    let movement: Vec<_> = input[0].chars().collect();

    let map: HashMap<_, _> = input[2..]
        .iter()
        .map(|row| row.split(" = ").collect_tuple().unwrap())
        .map(|(source, destination)| {
            let cleaned_input = destination.replace(['(', ')', ','], "");
            let (left, right) = cleaned_input
                .split_ascii_whitespace()
                .collect_tuple()
                .unwrap();
            (
                source.to_owned(),
                Destination {
                    left: left.to_owned(),
                    right: right.to_owned(),
                },
            )
        })
        .collect();

    let mut steps = 0;
    let mut node = map.get(START_NODE).unwrap();
    for movement in movement.iter().cycle() {
        steps += 1;
        let next = node.next(movement);
        if next == FINISH_NODE {
            break;
        }
        node = map.get(next).unwrap();
    }

    println!("Two: {steps} | Elapsed: {:?}", now.elapsed());
}

fn main() {
    let input = input();
    // one(&input);
    two(&input);
}
