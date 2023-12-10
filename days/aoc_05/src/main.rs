use std::{io::BufRead, vec};

fn input() -> Vec<String> {
    let stdin = std::io::stdin();
    stdin.lock().lines().map_while(Result::ok).collect()
}

enum Type<'a> {
    Name(&'a String),
    Mapping((u64, u64, u64)),
}

fn one(input: &[String]) {
    use itertools::Itertools;

    let now = std::time::Instant::now();

    let mut seeds: Vec<_> = input[0]
        .trim_start_matches("seeds: ")
        .split(' ')
        .map(|seed_number| seed_number.parse::<u64>().unwrap())
        .collect();

    let input: &Vec<Type> = &input[2..]
        .iter()
        .filter(|row| !row.is_empty())
        .map(|row| {
            if row.contains("map") {
                Type::Name(row)
            } else {
                Type::Mapping(
                    row.split_ascii_whitespace()
                        .map(|num| num.parse::<u64>().unwrap())
                        .collect_tuple()
                        .unwrap(),
                )
            }
        })
        .collect();

    let mut moved = vec![false; seeds.len()];
    for row in input {
        match row {
            Type::Name(_) => {
                for v in &mut moved {
                    *v = false;
                }
            }
            Type::Mapping((destination, source, range)) => {
                for (idx, seed) in seeds.iter_mut().enumerate() {
                    if *seed >= *source && *seed <= source + (range - 1) && !moved[idx] {
                        *seed = destination + (*seed - source);
                        moved[idx] = true;
                    }
                }
            }
        }
    }

    let min = seeds.iter().min().unwrap();
    println!("One: {min} | Elapsed: {:?}", now.elapsed());
}

fn two(input: &[String]) {
    use itertools::Itertools;

    let now = std::time::Instant::now();

    let mut seeds: Vec<_> = input[0]
        .trim_start_matches("seeds: ")
        .split(' ')
        .map(|seed_number| seed_number.parse::<u64>().unwrap())
        .collect();

    let input: &Vec<Type> = &input[2..]
        .iter()
        .filter(|row| !row.is_empty())
        .map(|row| {
            if row.contains("map") {
                Type::Name(row)
            } else {
                Type::Mapping(
                    row.split_ascii_whitespace()
                        .map(|num| num.parse::<u64>().unwrap())
                        .collect_tuple()
                        .unwrap(),
                )
            }
        })
        .collect();

    let mut moved = vec![false; seeds.len()];
    for (idx, row) in input.iter().enumerate() {
        match row {
            Type::Name(name) => {
                for v in &mut moved {
                    *v = false;
                }
                // println!("\n>>>> {name} <<<<");
            }
            Type::Mapping((destination, source, range)) => {
                // println!("Seed:  {seeds:?} Mapping: {{ Destination {destination} Source: {source} Range: {range} }}");
                // println!("==========================================");

                for (idx, seed) in seeds.iter_mut().enumerate() {
                    if *seed >= *source && *seed <= source + (range - 1) && !moved[idx] {
                        // println!(
                        //     "MOVE => Seed: {seed} Soure: {}..{} Destination {destination} Diff {}",
                        //     source,
                        //     source + range - 1,
                        //     *seed - source
                        // );
                        *seed = destination + (*seed - source);
                        moved[idx] = true;
                        // println!("moved {moved:?}");
                    } else {
                        // println!(
                        //     "STAY => Seed: {seed} Soure: {}..{} Destination {destination}",
                        //     source,
                        //     source + range - 1,
                        // );
                    }
                }
                // println!("Seed:  {seeds:?}");

                // println!("==========================================");
            }
        }
    }

    println!("Two: 0 | Elapsed: {:?}", now.elapsed());
}

fn main() {
    let input = input();
    one(&input);
    two(&input);
}
