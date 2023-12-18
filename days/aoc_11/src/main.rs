//! Part 1:
//! Part 2:

use std::{collections::HashSet, io::BufRead};

fn input() -> Vec<Vec<char>> {
    let stdin = std::io::stdin();
    stdin
        .lock()
        .lines()
        .map_while(Result::ok)
        .map(|row| row.chars().collect::<Vec<_>>())
        .collect()
}

fn expand_horizontal_empty_space(mut input: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let to_insert: Vec<usize> = input
        .iter()
        .enumerate()
        .filter_map(|(idx, row)| {
            if row.iter().filter(|c| c == &&'#').count() == 0 {
                Some(idx)
            } else {
                None
            }
        })
        .collect();
    // Remember offset
    for (idx, row) in to_insert.iter().enumerate() {
        input.insert(row + idx, ".".repeat(input[0].len()).chars().collect())
    }
    input
}

fn expand_vertical_space(mut input: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut to_insert = Vec::new();
    for (char_idx, _) in input[0].iter().enumerate() {
        let mut num_galaxies = 0;
        for (row_idx, _) in input.iter().enumerate() {
            if input[row_idx][char_idx] == '#' {
                num_galaxies += 1;
            }
        }
        if num_galaxies == 0 {
            to_insert.push(char_idx);
        }
    }

    for (idx, char_idx) in to_insert.iter().enumerate() {
        for row in &mut input {
            row.insert(char_idx + idx, '.')
        }
    }

    input
}

fn calculate_distance(a_loc: &(usize, usize), b_loc: &(usize, usize)) -> isize {
    // Signed integer for negative distances
    let a_loc = (a_loc.0 as isize, a_loc.1 as isize);
    let b_loc = (b_loc.0 as isize, b_loc.1 as isize);

    isize::abs(b_loc.0 - a_loc.0) + isize::abs(b_loc.1 - a_loc.1)
}

fn one(input: Vec<Vec<char>>) {
    let now = std::time::Instant::now();
    let input = expand_vertical_space(expand_horizontal_empty_space(input));

    let galaxies: Vec<_> = input
        .iter()
        .enumerate()
        .flat_map(|(row_idx, row)| {
            row.iter().enumerate().filter_map(move |(char_idx, c)| {
                if *c == '#' {
                    Some((row_idx, char_idx))
                } else {
                    None
                }
            })
        })
        .enumerate()
        .map(|(idx, galaxy)| (idx + 1, galaxy))
        .collect();

    let mut seen = HashSet::with_capacity(galaxies.len().pow(2));
    let total_distance: isize = galaxies
        .iter()
        .flat_map(|(a, a_loc)| {
            galaxies
                .iter()
                .filter_map(|(b, b_loc)| {
                    if seen.insert((a, b)) && seen.insert((b, a)) {
                        Some(calculate_distance(a_loc, b_loc))
                    } else {
                        None
                    }
                })
                .collect::<Vec<isize>>()
        })
        .sum();

    println!("One: {total_distance} | Elapsed: {:?}", now.elapsed());
}

fn two(input: Vec<Vec<char>>) {
    let now = std::time::Instant::now();
    let input = expand_vertical_space(expand_horizontal_empty_space(input));
    // for row in &input {
    //     for c in row {
    //         print!("{c}");
    //     }
    //     println!();
    // }

    let galaxies: Vec<_> = input
        .iter()
        .enumerate()
        .flat_map(|(row_idx, row)| {
            row.iter().enumerate().filter_map(move |(char_idx, c)| {
                if *c == '#' {
                    Some((row_idx, char_idx))
                } else {
                    None
                }
            })
        })
        .enumerate()
        .map(|(idx, galaxy)| (idx + 1, galaxy))
        .collect();

    // Distances -> Calculate diff in X and Y height.
    // let _total_distance = 0;
    // let mut distances = Vec::new();
    // let mut seen = HashSet::with_capacity(galaxies.len().pow(2));
    // for (a, a_loc) in &galaxies {
    //     for (b, b_loc) in &galaxies {
    //         if a != b && seen.insert((a, b)) && seen.insert((b, a)) {
    //             distances.push(((a, b), calculate_distance(a_loc, b_loc)));
    //         }
    //     }
    // }

    // // let total_distance: isize = distances.iter().map(|(_, distance)| distance).sum();

    let mut seen = HashSet::with_capacity(galaxies.len().pow(2));
    let total_distance: isize = galaxies
        .iter()
        .flat_map(|(a, a_loc)| {
            galaxies
                .iter()
                .filter_map(|(b, b_loc)| {
                    if seen.insert((a, b)) && seen.insert((b, a)) {
                        Some(calculate_distance(a_loc, b_loc))
                    } else {
                        None
                    }
                })
                .collect::<Vec<isize>>()
        })
        .sum();

    // for distance in &distances {
    //     println!("{distance:?}");
    // }

    // println!("num: {}", distances.len());

    println!("Two: {total_distance} | Elapsed: {:?}", now.elapsed());
}

fn main() {
    let input = input();
    one(input.clone());
    two(input);
}
