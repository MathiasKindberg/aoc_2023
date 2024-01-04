//! Part 1: 50603
//! Part 2: 96556251590677

use std::io::BufRead;

#[derive(Debug)]
struct Command {
    direction: Direction,
    steps: isize,
    color: String,
}

impl Command {
    fn color(&self) -> Command {
        let (num, direction) = self.color.split_at(5);
        Command {
            direction: match direction {
                "0" => Direction::Right,
                "1" => Direction::Down,
                "2" => Direction::Left,
                "3" => Direction::Up,
                unknown => unreachable!("Unknown direction: {unknown}"),
            },
            steps: isize::from_str_radix(num, 16).unwrap(),
            color: self.color.clone(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

fn input() -> Vec<Command> {
    let stdin = std::io::stdin();
    stdin
        .lock()
        .lines()
        .map_while(Result::ok)
        .map(|row| {
            let (direction, rest) = row.split_once(' ').unwrap();
            let (steps, color) = rest.split_once(' ').unwrap();
            Command {
                direction: match direction {
                    "R" => Direction::Right,
                    "D" => Direction::Down,
                    "L" => Direction::Left,
                    "U" => Direction::Up,
                    unknown => unreachable!("Unknown direction: {unknown}"),
                },
                steps: steps.parse().unwrap(),
                color: color.replace(['(', ')', '#'], "").to_owned(),
            }
        })
        .collect()
}

/// Verified the input to not contain any 2 wide paths. We also do not
/// check to ensure we step out of bounds because we rely on the
/// dug to be a border.
/// ####
/// #··#
/// ·##· <--- The algorithm can't handle this case!
/// ·##·
/// #··#
/// ####
fn flood_fill(grid: &mut Vec<Dug>, num_cols: isize, row: isize, col: isize) {
    grid[to_index(num_cols, col, row)] = Dug::Yay;

    // Right
    if grid[to_index(num_cols, col + 1, row)] == Dug::Nay {
        flood_fill(grid, num_cols, row, col + 1)
    }
    // Down
    if grid[to_index(num_cols, col, row + 1)] == Dug::Nay {
        flood_fill(grid, num_cols, row + 1, col)
    }
    // Left
    if grid[to_index(num_cols, col - 1, row)] == Dug::Nay {
        flood_fill(grid, num_cols, row, col - 1)
    }
    // Up
    if grid[to_index(num_cols, col, row - 1)] == Dug::Nay {
        flood_fill(grid, num_cols, row - 1, col)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dug {
    Yay,
    Nay,
}

impl std::fmt::Display for Dug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Dug::Yay => write!(f, "#"),
            Dug::Nay => write!(f, "·"), // Switch to interpunct "."
        }
    }
}

fn to_index(num_cols: isize, col: isize, row: isize) -> usize {
    // Should never turn negative since we offset
    (row * num_cols + col) as usize
}

fn one(input: &[Command]) {
    let now = std::time::Instant::now();
    // 1. Find boundaries.
    let mut min_row = 0;
    let mut min_col = 0;
    let mut max_row = 0;
    let mut max_col = 0;

    let mut row = 0;
    let mut col = 0;

    for cmd in input {
        match cmd.direction {
            Direction::Right => col += cmd.steps,
            Direction::Down => row += cmd.steps,
            Direction::Left => col -= cmd.steps,
            Direction::Up => row -= cmd.steps,
        }

        min_row = row.min(min_row);
        min_col = col.min(min_col);

        max_row = row.max(max_row);
        max_col = col.max(max_col);
    }

    let num_rows = max_row + min_row.abs() + 1;
    let num_cols = max_col + min_col.abs() + 1;

    // Get our starting offset to ensure we never leave the grid.
    row = min_row.abs();
    col = min_col.abs();

    let mut grid = vec![Dug::Nay; (num_rows * num_cols) as usize];

    for cmd in input {
        // Could reverse this for efficiency, but oh well. This is easier.
        for _ in 0..cmd.steps {
            grid[to_index(num_cols, col, row)] = Dug::Yay;

            match cmd.direction {
                Direction::Right => col += 1,
                Direction::Down => row += 1,
                Direction::Left => col -= 1,
                Direction::Up => row -= 1,
            }
        }
    }

    // Find edge to flood fill from.

    // 1 row down because we know that we have a border on the topmost row
    row = 1;
    col = 0;

    let mut found_dug = false;
    for _ in 0..num_cols {
        if found_dug && grid[to_index(num_cols, col, row)] == Dug::Nay {
            // Here we start
            break;
        } else if grid[to_index(num_cols, col, row)] == Dug::Yay {
            found_dug = true;
        }
        col += 1;
    }

    flood_fill(&mut grid, num_cols, row, col);
    let sum = grid.iter().filter(|&&tile| tile == Dug::Yay).count();
    println!("One: {sum} | Elapsed: {:?}", now.elapsed());
}

#[derive(Debug, Clone, Copy)]
struct Coord {
    col: isize,
    row: isize,
}

// Now the naive implementation tries to allocate too much memory.
fn two(input: &[Command]) {
    let now = std::time::Instant::now();

    let input: Vec<_> = input.iter().map(|cmd| cmd.color()).collect();

    let mut row: isize = 0;
    let mut col: isize = 0;

    // Determine coordinates for each point
    let mut points: Vec<Coord> = Vec::with_capacity(input.len());
    let start = Coord { row: 0, col: 0 };
    points.push(start);

    for cmd in input {
        match cmd.direction {
            Direction::Right => col += cmd.steps,
            Direction::Down => row += cmd.steps,
            Direction::Left => col -= cmd.steps,
            Direction::Up => row -= cmd.steps,
        }
        points.push(Coord { row, col });
    }

    let perimeter = perimeter(&points);
    let area = shoelace_formula(&points) as isize;
    let interior: isize = area + (perimeter / 2) + 1;

    println!("Sum: {interior} | Elapsed: {:?}", now.elapsed());
}

/// Idiomatic implementation of shoelace formula. Requires a
///
/// https://en.wikipedia.org/wiki/Shoelace_formula
#[allow(dead_code)]
fn shoelace_formula(points: &[Coord]) -> f64 {
    points
        .windows(2)
        .map(|window| {
            let x1 = window[0].col;
            let y1 = window[0].row;
            let x2 = window[1].col;
            let y2 = window[1].row;

            (x1 * y2) - (x2 * y1)
        })
        .sum::<isize>() as f64
        / 2.0
}

fn euclidian_distance(a: Coord, b: Coord) -> f64 {
    let x1 = a.col;
    let y1 = a.row;
    let x2 = b.col;
    let y2 = b.row;
    f64::abs(f64::sqrt(((x2 - x1).pow(2) + (y2 - y1).pow(2)) as f64))
}

fn perimeter(points: &[Coord]) -> isize {
    let mut perimieter = 0;
    for window in points.windows(2) {
        perimieter += euclidian_distance(window[0], window[1]) as isize
    }

    // Need to include the distance from the last point to the first point
    // perimieter + euclidian_distance(*points.last().unwrap(), *points.first().unwrap()) as isize
    perimieter
}

fn main() {
    let input = input();
    one(&input);
    two(&input);
}
