//! Part 1: 34918
//! Part 2:

use std::io::BufRead;

fn input() -> Vec<String> {
    let stdin = std::io::stdin();
    stdin.lock().lines().map_while(Result::ok).collect()
}

// https://stackoverflow.com/questions/64498617/how-to-transpose-a-vector-of-vectors-in-rust
fn transpose2<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

/// Validate if split is a true mirroring point
fn validate_mirror_index(map: &[Vec<char>], idx: usize, split_distance: isize) -> bool {
    // We do not need to be equal if the split line does not have a matching row outside.
    // + 1 to put the split line in between two rows
    if idx as isize - split_distance < 0 || idx as isize + 1 + split_distance >= map.len() as isize
    {
        return true;
    }

    if map[idx - split_distance as usize]
        .iter()
        .eq(map[idx + 1 + split_distance as usize].iter())
    {
        validate_mirror_index(map, idx, split_distance + 1)
    } else {
        false
    }
}

/// Finds all potential mirror splits
fn find_mirror_index(map: &[Vec<char>]) -> Option<usize> {
    let potential_mirror_indexes: Vec<_> = map
        .windows(2)
        .enumerate()
        .filter_map(|(idx, window)| {
            if window[0].iter().eq(window[1].iter()) {
                Some(idx)
            } else {
                None
            }
        })
        .collect();

    for idx in potential_mirror_indexes {
        if validate_mirror_index(map, idx, 1) {
            return Some(idx + 1);
        }
    }
    None
}

/// To solve:
/// 1. Find which line splits the input, horizontal or vertical.
/// 2. Count lines to the left and above and summarize.
fn one(input: &[String]) {
    let now = std::time::Instant::now();
    let mut sum = 0;
    let input: Vec<_> = input
        .iter()
        .map(|row| row.chars().collect::<Vec<_>>())
        .collect();
    let input: Vec<_> = input.split(|row| row.is_empty()).collect();

    for map in &input {
        let horizontal = find_mirror_index(map);
        let vertical = find_mirror_index(&transpose2(map.to_vec()));

        assert_ne!(
            horizontal.is_some(),
            vertical.is_some(),
            "We should find either a horizontal or vertical split"
        );

        sum += horizontal.unwrap_or(0) * 100 + vertical.unwrap_or(0);
    }

    println!("One: {sum} | Elapsed: {:?}", now.elapsed());
}
fn two(_input: &[String]) {
    let now = std::time::Instant::now();
    let sum = 0;

    println!("Two: {sum} | Elapsed: {:?}", now.elapsed());
}

fn main() {
    let input = input();
    one(&input);
    two(&input);
}
