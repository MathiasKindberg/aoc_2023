//! Part 1: 11309
//! Part 2: 13740108158591

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

fn one(input: &[String]) -> anyhow::Result<()> {
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
    let mut node = map
        .get(START_NODE)
        .ok_or(anyhow::anyhow!("Expected to find starting node"))?;
    for movement in movement.iter().cycle() {
        steps += 1;
        let next = node.next(movement);
        if next == FINISH_NODE {
            break;
        }
        node = map.get(next).unwrap();
    }

    println!("One: {steps} | Elapsed: {:?}", now.elapsed());
    Ok(())
}
fn two(input: &[String]) {
    use itertools::Itertools;

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

    let mut start_nodes: Vec<_> = map.keys().filter(|node| node.ends_with('A')).collect();

    let mut nodes: Vec<_> = start_nodes
        .iter()
        .map(|node| (map.get(*node).unwrap(), None, None))
        .collect();

    let mut steps: usize = 0;
    let mut finished_nodes = 0;

    for (idx, movement) in movement.iter().cycle().enumerate() {
        steps += 1;

        // Could be cleaned up, but good enough.
        for (node, found_z, steps_to_z) in &mut nodes {
            if found_z.is_none() {
                let next = node.next(movement);
                *node = map.get(next).unwrap();

                if next.ends_with("Z") {
                    *found_z = Some(next);
                    *steps_to_z = Some(steps);
                    finished_nodes += 1;
                }
            }
        }

        if finished_nodes == nodes.len() {
            break;
        }
    }

    let lcm = nodes
        .iter()
        .fold(None, |mut lcm_acc, (_, _, steps_to_z)| {
            let steps_to_z = steps_to_z.expect("To found steps to all nodes");
            match lcm_acc {
                None => lcm_acc = Some(steps_to_z),
                Some(curr_lcm) => lcm_acc = Some(lcm(curr_lcm, steps_to_z)),
            }
            lcm_acc
        })
        .unwrap();

    dbg!(&lcm);

    println!("Two: {lcm} | Elapsed: {:?}", now.elapsed());
}

// Could use libraries but this is easy enough.
// https://rosettacode.org/wiki/Least_common_multiple
fn gcd(a: usize, b: usize) -> usize {
    use std::cmp::{max, min};

    match ((a, b), (a & 1, b & 1)) {
        ((x, y), _) if x == y => y,
        ((0, x), _) | ((x, 0), _) => x,
        ((x, y), (0, 1)) | ((y, x), (1, 0)) => gcd(x >> 1, y),
        ((x, y), (0, 0)) => gcd(x >> 1, y >> 1) << 1,
        ((x, y), (1, 1)) => {
            let (x, y) = (min(x, y), max(x, y));
            gcd((y - x) >> 1, x)
        }
        _ => unreachable!(),
    }
}

// https://rosettacode.org/wiki/Least_common_multiple
fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn main() {
    let input = input();
    let _ = one(&input).map_err(|err| println!("ERR: Error in first task: {err}"));
    two(&input);
}
