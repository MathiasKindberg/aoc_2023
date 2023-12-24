//! Part 1: 34918
//! Part 2: 33054

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
fn validate_reflection_index(
    map: &[Vec<char>],
    idx: usize,
    split_distance: isize,
    max_diff: usize,
) -> bool {
    // We do not need to be equal if the split line does not have a matching row outside.
    // + 1 to put the split line in between two rows
    if idx as isize - split_distance < 0 || idx as isize + 1 + split_distance >= map.len() as isize
    {
        // Only return true if we have used up our expected difference. Otherwise we will find the
        // original line that does not need a difference
        return max_diff == 0;
    }

    let diff = map[idx - split_distance as usize]
        .iter()
        .zip(map[idx + 1 + split_distance as usize].iter())
        .filter(|(a, b)| a != b)
        .count();

    if diff <= max_diff {
        validate_reflection_index(map, idx, split_distance + 1, max_diff - diff)
    } else {
        false
    }
}

fn find_reflection_index(map: &[Vec<char>], diff: usize) -> Option<usize> {
    let potential_mirror_indexes: Vec<_> = map
        .windows(2)
        .enumerate()
        .filter_map(|(idx, window)| {
            let found_diff = window[0]
                .iter()
                .zip(window[1].iter())
                .filter(|(a, b)| a != b)
                .count();
            if found_diff <= diff {
                Some((idx, found_diff))
            } else {
                None
            }
        })
        .collect();

    for (idx, found_diff) in potential_mirror_indexes {
        if validate_reflection_index(map, idx, 1, diff - found_diff) {
            return Some(idx + 1);
        }
    }
    None
}

fn one(input: &[String]) {
    let now = std::time::Instant::now();
    let mut sum = 0;
    let input: Vec<_> = input
        .iter()
        .map(|row| row.chars().collect::<Vec<_>>())
        .collect();
    let input: Vec<_> = input.split(|row| row.is_empty()).collect();

    for map in &input {
        let horizontal = find_reflection_index(map, 0);
        let vertical = find_reflection_index(&transpose2(map.to_vec()), 0);

        assert_ne!(
            horizontal.is_some(),
            vertical.is_some(),
            "We should find either a horizontal or vertical split"
        );

        sum += horizontal.unwrap_or(0) * 100 + vertical.unwrap_or(0);
    }

    println!("One: {sum} | Elapsed: {:?}", now.elapsed());
}

fn two(input: Vec<String>) {
    let now = std::time::Instant::now();
    let mut sum = 0;
    let input: Vec<Vec<char>> = input
        .iter()
        .cloned()
        .map(|row| row.chars().collect::<Vec<_>>())
        .collect();
    let input: Vec<Vec<Vec<char>>> = input
        .split(|row| row.is_empty())
        .map(|row| row.to_vec())
        .collect();

    for map in &input {
        let horizontal = find_reflection_index(map, 1);
        let vertical = find_reflection_index(&transpose2(map.to_vec()), 1);

        assert_ne!(
            horizontal.is_some(),
            vertical.is_some(),
            "We should find either a horizontal or vertical split"
        );

        sum += horizontal.unwrap_or(0) * 100 + vertical.unwrap_or(0);
    }

    println!("Two: {sum} | Elapsed: {:?}", now.elapsed());
}

fn main() {
    let input = input();
    one(&input);
    two(input);
}
