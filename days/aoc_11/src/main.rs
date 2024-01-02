//! Part 1: 9648398
//! Part 2: 618800410814

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
        input.insert(row + idx, ".".repeat(input[0].len()).chars().collect());
    }
    input
}

fn expand_vertical_space(mut input: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut to_insert = Vec::new();
    for (col_idx, _) in input[0].iter().enumerate() {
        let mut num_galaxies = 0;
        for (row_idx, _) in input.iter().enumerate() {
            if input[row_idx][col_idx] == '#' {
                num_galaxies += 1;
            }
        }
        if num_galaxies == 0 {
            to_insert.push(col_idx);
        }
    }

    for (idx, char_idx) in to_insert.iter().enumerate() {
        for row in &mut input {
            row.insert(char_idx + idx, '.');
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
            row.iter().enumerate().filter_map(move |(col_idx, c)| {
                if *c == '#' {
                    Some((row_idx, col_idx))
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
    const UNIVERSE_EXPANSION: usize = 1000000;
    // Need this because we are 0 indexed.
    const OFFSET: usize = UNIVERSE_EXPANSION - 1;
    let now = std::time::Instant::now();

    let mut row_expansion = 0;
    let mut row_expansions = vec![];
    let mut col_expansion = 0;
    let mut col_expansions = vec![];

    for row in &input {
        row_expansions.push(row_expansion);
        if row.iter().filter(|c| c == &&'#').count() == 0 {
            row_expansion += OFFSET;
        }
    }

    for (col_idx, _) in input[0].iter().enumerate() {
        let mut num = 0;
        for (row_idx, _) in input.iter().enumerate() {
            if input[row_idx][col_idx] == '#' {
                num += 1;
            }
        }

        col_expansions.push(col_expansion);
        if num == 0 {
            col_expansion += OFFSET;
        }
    }

    let galaxies: Vec<_> = input
        .iter()
        .enumerate()
        .flat_map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(col_idx, c)| {
                    if *c == '#' {
                        Some((
                            row_idx + row_expansions[row_idx],
                            col_idx + col_expansions[col_idx],
                        ))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .enumerate()
        .map(|(idx, galaxy)| (idx + 1, galaxy))
        .collect();

    for galaxy in &galaxies {
        println!("{galaxy:?}");
    }

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

    println!("Two: {total_distance} | Elapsed: {:?}", now.elapsed());
}

fn main() {
    let input = input();
    one(input.clone());
    two(input);
}
