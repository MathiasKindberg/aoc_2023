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
    let now = std::time::Instant::now();
    let sum: u64 = input
        .iter()
        .map(|row| row.split(": ").collect::<Vec<_>>())
        .filter_map(|row| {
            let game_id = row[0].trim_start_matches("Game ").parse::<u64>().unwrap();
            let sets = row[1].split("; ").collect::<Vec<_>>();
            for set in &sets {
                let cubes = set.split(", ").collect::<Vec<_>>();
                for cube in cubes {
                    let cube = cube.split(' ').collect::<Vec<_>>();
                    let (num, color) = (cube[0].parse::<u64>().unwrap(), cube[1]);

                    if num > *lookup_table().get(color).unwrap() {
                        return None;
                    }
                }
            }
            Some(game_id)
        })
        .sum();
    println!("One: {sum} | Elapsed: {:?}", now.elapsed());
}
fn two(_input: &[String]) {}

fn main() {
    let input = input();
    one(&input);
    two(&input);
}
