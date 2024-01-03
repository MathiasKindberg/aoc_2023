//! Part 1: 1244
//! Part 2:

use std::{collections::HashMap, io::BufRead};

fn input() -> Vec<Vec<usize>> {
    let stdin = std::io::stdin();
    stdin
        .lock()
        .lines()
        .map_while(Result::ok)
        .map(|row| {
            row.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct State {
    cost: usize,
    position: usize,
    direction: Direction,
    moves_in_direction: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct StateKey {
    position: usize,
    direction: Direction,
    moves_in_direction: usize,
}

impl From<State> for StateKey {
    fn from(value: State) -> Self {
        Self {
            position: value.position,
            direction: value.direction,
            moves_in_direction: value.moves_in_direction,
        }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Used to prevent us going more than 3 tiles straight.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Debug)]
struct Edge {
    node: usize,
    cost: usize,
    direction: Direction,
}

fn build_adjacency_list(input: &[usize], num_rows: usize, row_length: usize) -> Vec<Vec<Edge>> {
    let mut adjacency_list = Vec::with_capacity(input.len());

    for (node_idx, _) in input.iter().enumerate() {
        let row_idx = node_idx / row_length;
        let col_idx = node_idx % row_length;
        let mut node_adjacencies = Vec::with_capacity(4);

        // UP
        if row_idx > 0 {
            let up_idx = node_idx - row_length;
            node_adjacencies.push(Edge {
                node: up_idx,
                cost: input[up_idx],
                direction: Direction::Up,
            });
        }

        // DOWN
        if row_idx < num_rows - 1 {
            let down_idx = node_idx + row_length;
            node_adjacencies.push(Edge {
                node: down_idx,
                cost: input[down_idx],
                direction: Direction::Down,
            });
        }

        // RIGHT
        if col_idx < row_length - 1 {
            let right_idx = node_idx + 1;
            node_adjacencies.push(Edge {
                node: right_idx,
                cost: input[right_idx],
                direction: Direction::Right,
            });
        }

        // LEFT.
        if col_idx > 0 {
            let left_idx = node_idx - 1;
            node_adjacencies.push(Edge {
                node: left_idx,
                cost: input[left_idx],
                direction: Direction::Left,
            });
        }

        adjacency_list.push(node_adjacencies)
    }
    adjacency_list
}

fn dijkstra(
    adj_list: &[Vec<Edge>],
    start: usize,
    start_cost: usize,
    goal: usize,
    min_steps_in_direction: usize,
    max_steps_in_direction: usize,
) -> Option<usize> {
    let mut dist: HashMap<StateKey, _> = HashMap::new();
    let mut heap = std::collections::BinaryHeap::new();

    // We're at `start`, with some initial cost.
    let start1 = State {
        cost: 0,
        position: start,
        direction: Direction::Right,
        moves_in_direction: 1,
    };

    let start2 = State {
        cost: 0,
        position: start,
        direction: Direction::Down,
        moves_in_direction: 1,
    };

    dist.insert(start1.into(), start_cost);
    dist.insert(start2.into(), start_cost);
    heap.push(start1);
    heap.push(start2);

    while let Some(curr) = heap.pop() {
        if curr.position == goal {
            return Some(curr.cost);
        }

        // Important as we may have already found a better way
        if dist
            .get(&curr.into())
            .is_some_and(|known_cost| known_cost < &curr.cost)
        {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for edge in &adj_list[curr.position] {
            let next = State {
                cost: curr.cost + edge.cost,
                position: edge.node,
                direction: edge.direction,
                moves_in_direction: if curr.direction == edge.direction {
                    curr.moves_in_direction + 1
                } else {
                    1
                },
            };

            // P1 and p2
            if next.moves_in_direction > max_steps_in_direction
                || curr.direction.opposite() == next.direction
                || dist
                    .get(&next.into())
                    .is_some_and(|cost| *cost <= next.cost)
            {
                continue;
            }

            if next.direction != curr.direction && next.moves_in_direction < min_steps_in_direction
            {
                continue;
            }

            dist.insert(next.into(), next.cost);
            heap.push(next);
        }
    }

    // Goal not reachable
    None
}

fn solve(input: Vec<Vec<usize>>) {
    let now = std::time::Instant::now();

    let num_rows = input.len();
    let row_length = input[0].len();
    let input: Vec<_> = input.into_iter().flatten().collect();

    let adjacency_list = build_adjacency_list(&input, num_rows, row_length);

    let start_idx = 0;
    let end_idx = input.len() - 1;

    let cost = dijkstra(&adjacency_list, start_idx, input[start_idx], end_idx, 1, 3).unwrap();
    println!("One: {cost} | Elapsed: {:?}", now.elapsed());

    let cost = dijkstra(&adjacency_list, start_idx, input[start_idx], end_idx, 4, 10).unwrap();
    println!("Two: {cost} | Elapsed: {:?}", now.elapsed());
}

fn main() {
    let input = input();
    solve(input);
}
