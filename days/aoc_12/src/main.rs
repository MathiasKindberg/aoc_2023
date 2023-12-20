//! Part 1: 7350
//! Part 2: 200097286528151

use std::{collections::HashMap, io::BufRead};

fn input() -> Vec<String> {
    let stdin = std::io::stdin();
    stdin.lock().lines().map_while(Result::ok).collect()
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Record {
    springs: Vec<Spring>,
    counts: Vec<usize>,
}

/// Solution utilizing memoization to remember past inputs.
/// Inspired by: https://gist.github.com/icub3d/7aa45ca96ccb88ebf95b91d6a28eba74
fn valid_arrangements_memo<'a>(
    springs: &'a [Spring],
    counts: &'a [usize],
    memo: &mut HashMap<(&'a [Spring], &'a [usize]), usize>,
) -> usize {
    if let Some(arrangements) = memo.get(&(springs, counts)) {
        return *arrangements;
    }

    // If the count is empty and we have no more damage spring to assign then we have a valid arrangement
    if counts.is_empty() {
        let arrangements = match springs.iter().any(|spring| spring == &Spring::Damaged) {
            true => 0,
            false => 1,
        };
        memo.insert((springs, counts).clone(), arrangements);
        return arrangements;
    }

    // If we can't fit the remaining springs then we can't find any more valid
    // arrangements down this path.
    if springs.len() < counts.iter().sum::<usize>() + counts.len() - 1 {
        memo.insert((springs, counts).clone(), 0);
        return 0;
    }

    // Skip working on operational springs
    if springs[0] == Spring::Operational {
        let arrangements = valid_arrangements_memo(&springs[1..], counts, memo);
        memo.insert((springs, counts).clone(), arrangements);
        return arrangements;
    }

    let current_group_length = counts[0];
    let all_non_operational = springs[0..current_group_length]
        .iter()
        .all(|c| c != &Spring::Operational);
    let group_end = (current_group_length + 1).min(springs.len());

    let mut arrangements = if all_non_operational
        && ((springs.len() > current_group_length
            && springs[current_group_length] != Spring::Damaged)
            || springs.len() <= current_group_length)
    {
        valid_arrangements_memo(&springs[group_end..], &counts[1..], memo)
    } else {
        0
    };

    if springs[0] == Spring::Unknown {
        arrangements += valid_arrangements_memo(&springs[1..], counts, memo)
    }

    memo.insert((springs, counts).clone(), arrangements);

    arrangements
}

impl Record {
    /// Naive solution testing every combination and not memoizing anything.
    fn valid_arrangements(&mut self) -> usize {
        if let Some(index) = self
            .springs
            .iter()
            .position(|spring| spring == &Spring::Unknown)
        {
            self.springs[index] = Spring::Operational;
            let operational_arrangements = self.valid_arrangements();

            self.springs[index] = Spring::Damaged;
            let damaged_arrangements = self.valid_arrangements();

            self.springs[index] = Spring::Unknown;

            operational_arrangements + damaged_arrangements
        } else {
            self.is_valid().into()
        }
    }

    fn is_valid(&self) -> bool {
        use itertools::Itertools;

        self.springs
            .iter()
            .group_by(|item| *item)
            .into_iter()
            .filter_map(|(key, group)| {
                if *key == Spring::Damaged {
                    Some(group.count())
                } else {
                    None
                }
            })
            .eq(self.counts.iter().copied())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Spring {
    Operational,
    Unknown,
    Damaged,
}

impl std::fmt::Display for Spring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Spring::Operational => write!(f, "."),
            Spring::Unknown => write!(f, "?"),
            Spring::Damaged => write!(f, "#"),
        }
    }
}

fn one(input: &[String]) {
    let now = std::time::Instant::now();
    let mut input: Vec<_> = input
        .iter()
        .map(|row| row.split_once(' ').unwrap())
        .map(|(springs, counts)| {
            (
                springs
                    .chars()
                    .map(|c| match c {
                        '.' => Spring::Operational,
                        '?' => Spring::Unknown,
                        '#' => Spring::Damaged,
                        _ => unreachable!(),
                    })
                    .collect::<Vec<_>>(),
                counts
                    .split(',')
                    .map(|num| num.parse::<usize>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .map(|(springs, counts)| Record { springs, counts })
        .collect();

    let valid_arrangements: usize = input
        .iter_mut()
        .map(|record| record.valid_arrangements())
        .sum();

    println!("One: {valid_arrangements} | Elapsed: {:?}", now.elapsed());
}

fn two(input: &[String]) {
    let now = std::time::Instant::now();

    let mut input: Vec<_> = input
        .iter()
        .map(|row| row.split_once(' ').unwrap())
        .map(|(springs, counts)| {
            (
                springs
                    .chars()
                    .map(|c| match c {
                        '.' => Spring::Operational,
                        '?' => Spring::Unknown,
                        '#' => Spring::Damaged,
                        _ => unreachable!(),
                    })
                    .collect::<Vec<_>>(),
                counts
                    .split(',')
                    .map(|num| num.parse::<usize>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .map(|(mut springs, mut counts)| {
            springs = springs
                .iter()
                .copied()
                .chain([Spring::Unknown])
                .cycle()
                // +4 since we are adding 4 unknowns in the middle of the 5 groups.
                .take(springs.len() * 5 + 4)
                .collect();

            counts = counts
                .iter()
                .copied()
                .cycle()
                .take(counts.len() * 5)
                .collect();
            (springs, counts)
        })
        .map(|(springs, counts)| Record { springs, counts })
        .collect();

    let valid_arrangements: usize = input
        .iter_mut()
        .map(|record| {
            let mut memo = HashMap::new();
            valid_arrangements_memo(&record.springs, &record.counts, &mut memo)
        })
        .sum();

    println!("Two: {valid_arrangements} | Elapsed: {:?}", now.elapsed());
}

fn main() {
    let input = input();
    one(&input);
    two(&input);
}
