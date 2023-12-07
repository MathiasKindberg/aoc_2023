//! One: 2505
//! Two: 70265

use std::collections::HashMap;
use std::io::BufRead;
use std::sync::OnceLock;

fn input() -> Vec<String> {
    let stdin = std::io::stdin();
    stdin.lock().lines().map_while(Result::ok).collect()
}

fn lookup_table() -> &'static HashMap<&'static str, u64> {
    static HASHMAP: OnceLock<HashMap<&str, u64>> = OnceLock::new();

    HASHMAP.get_or_init(|| HashMap::from_iter(LOOKUP_TABLE.iter().cloned()))
}

const LOOKUP_TABLE: &[(&str, u64)] = &[("red", 12), ("green", 13), ("blue", 14)];

fn one(input: &[String]) {
    use itertools::Itertools;

    let now = std::time::Instant::now();
    let sum: u64 = input
        .iter()
        .map(|row| row.split(": ").collect_tuple().unwrap())
        .map(|(game_id, games)| {
            (
                game_id.trim_start_matches("Game ").parse::<u64>().unwrap(),
                games.replace(';', ","),
            )
        })
        .filter_map(|(game_id, set)| {
            let set = set.split(", ").collect::<Vec<_>>();
            for cube in set {
                let cube = cube.split(' ').collect::<Vec<_>>();
                let (num, color) = (cube[0].parse::<u64>().unwrap(), cube[1]);
                if num > *lookup_table().get(color).unwrap() {
                    return None;
                }
            }
            Some(game_id)
        })
        .sum();

    println!("One: {sum} | Elapsed: {:?}", now.elapsed());
}

fn two(input: &[String]) {
    let now = std::time::Instant::now();
    let sum: u64 = input
        .iter()
        .map(|row| row.split(": ").skip(1).last().unwrap())
        .map(|game| game.replace(';', ","))
        .map(|game| {
            let mut max_of_each_color = HashMap::with_capacity(3);
            for cube in game.split(", ") {
                let cube = cube.split(' ').collect::<Vec<_>>();
                let (num, color) = (cube[0].parse::<u64>().unwrap(), cube[1]);

                max_of_each_color
                    .entry(color)
                    .and_modify(|val: &mut u64| *val = std::cmp::max(*val, num))
                    .or_insert(num);
            }
            max_of_each_color
                .iter()
                .fold(None, |acc, (_, value)| match acc {
                    Some(acc) => Some(acc * value),
                    None => Some(*value),
                })
                .unwrap()
        })
        .sum();

    println!("Two: {sum} | Elapsed: {:?}", now.elapsed());
}

fn main() {
    let input = input();
    one(&input);
    two(&input);
}
