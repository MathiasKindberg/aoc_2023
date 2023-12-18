//! Part 1:
//! Part 2:

use std::io::BufRead;

fn input() -> Vec<String> {
    let stdin = std::io::stdin();
    stdin.lock().lines().map_while(Result::ok).collect()
}

fn is_valid(geometry: &str, nums: &[usize]) -> bool {
    for num in nums {}
    false
}

fn one(input: &[String]) {
    use itertools::Itertools;
    let now = std::time::Instant::now();
    let sum = 0;
    // let input: Vec<(Vec<char>, Vec<usize>)> = input
    let input: Vec<(_, Vec<usize>)> = input
        .iter()
        .map(|row| row.split_ascii_whitespace().collect_tuple().unwrap())
        .map(|(groups, numbers)| {
            (
                groups.chars().map(|c| match c {
                    '?' => 
                }).collect::<Vec<_>>(),
                // groups,
                numbers
                    .split(',')
                    .map(|num| num.parse::<usize>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .collect();

    // println!("{input:#?}");
    let mut result = Vec::new();
    for (geometry, nums) in input {
        for char in geometry.chars() {}
        let num_arrangements = 0;

        println!("{geometry:?} {nums:?}");

        break;
        // result.clear();
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
