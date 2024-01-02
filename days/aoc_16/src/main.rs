//! Part 1:
//! Part 2:

use std::{collections::HashSet, io::BufRead, vec};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TileType {
    Empty,

    /// '|'
    VerticalSplitter,
    /// '-*
    HorizontalSplitter,

    /// '/'
    ForwardMirror,

    /// '\'
    BackwardMirror,

    /// '#'
    Wall,
}

#[derive(Debug, Clone, Copy)]
struct Tile {
    energizations: u64,
    tile_type: TileType,
}

#[derive(Debug)]
struct LightRay {
    location: num::Complex<i64>,
    movement_vector: num::Complex<i64>,
}

/// '/'
const FORWARD_MIRROR_REFLECTION: num::Complex<i64> = num::Complex::new(0, -1);

/// '\'
const BACKWARD_MIRROR_REFLECTION: num::Complex<i64> = num::Complex::new(0, 1);

const ENTRY_LOCATION: num::Complex<i64> = num::Complex::new(1, 1);
const ENTRY_MOVEMENT_VECTOR: num::Complex<i64> = num::Complex::new(1, 0);

/// Movement directions
const UP: num::Complex<i64> = num::Complex::new(0, -1);
const DOWN: num::Complex<i64> = num::Complex::new(0, 1);
const RIGHT: num::Complex<i64> = num::Complex::new(1, 0);
const LEFT: num::Complex<i64> = num::Complex::new(-1, 0);

impl LightRay {
    /// Interact with a tile returning the resulting
    /// light ray. Extra light rays are created when
    /// interacting with the splitters.
    fn interact(
        mut self,
        tile: &mut Tile,
        cycle_detection: &mut HashSet<(num::Complex<i64>, num::Complex<i64>)>,
    ) -> Vec<LightRay> {
        if !cycle_detection.insert((self.location, self.movement_vector)) {
            return vec![];
        }

        // Don't count our extra layer.
        if tile.tile_type != TileType::Wall {
            tile.energizations += 1;
        }

        match tile.tile_type {
            // Simple way to ensure light rays leaving the 2d space
            // dies.
            TileType::Wall => {
                vec![]
            }

            TileType::Empty => vec![self],

            TileType::ForwardMirror => {
                let reflection =
                    if self.movement_vector.re.abs() == 1 && self.movement_vector.im.abs() == 0 {
                        // →, ←
                        FORWARD_MIRROR_REFLECTION
                    } else {
                        // ↑, ↓
                        -FORWARD_MIRROR_REFLECTION
                    };

                self.movement_vector *= reflection;
                vec![self]
            }

            TileType::BackwardMirror => {
                let reflection =
                    if self.movement_vector.re.abs() == 1 && self.movement_vector.im.abs() == 0 {
                        // →, ←
                        BACKWARD_MIRROR_REFLECTION
                    } else {
                        // ↑, ↓
                        -BACKWARD_MIRROR_REFLECTION
                    };

                self.movement_vector *= reflection;
                vec![self]
            }

            TileType::VerticalSplitter => {
                // Horizontal movement. Re part > 0, Im part = 0
                // <--------->
                if self.movement_vector.re.abs() == 1 && self.movement_vector.im == 0 {
                    let resulting_rays = vec![
                        LightRay {
                            location: self.location,
                            movement_vector: UP,
                        },
                        LightRay {
                            location: self.location,
                            movement_vector: DOWN,
                        },
                    ];
                    assert_eq!(resulting_rays.len(), 2);
                    resulting_rays
                } else {
                    // Approaching from pointy end => Do nothing.
                    vec![self]
                }
            }
            TileType::HorizontalSplitter => {
                // Vertical movement. Re part = 0, Im part > 0
                // ↑
                // ↓
                if self.movement_vector.re == 0 && self.movement_vector.im.abs() == 1 {
                    let resulting_rays = vec![
                        LightRay {
                            location: self.location,
                            movement_vector: RIGHT,
                        },
                        LightRay {
                            location: self.location,
                            movement_vector: LEFT,
                        },
                    ];
                    resulting_rays
                } else {
                    // Approaching from pointy end => Do nothing.
                    vec![self]
                }
            }
        }
    }
}

impl std::fmt::Display for TileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TileType::Wall => write!(f, "#"),
            TileType::Empty => write!(f, "·"), // Switch to interpunct "."
            TileType::VerticalSplitter => write!(f, "|"),
            TileType::HorizontalSplitter => write!(f, "-"),
            TileType::ForwardMirror => write!(f, "/"),
            TileType::BackwardMirror => write!(f, "\\"),
        }
    }
}

fn input() -> Vec<Vec<Tile>> {
    let stdin = std::io::stdin();
    let input: Vec<_> = stdin
        .lock()
        .lines()
        .map_while(Result::ok)
        .map(|row| row.chars().collect::<Vec<_>>())
        .collect();

    // Pad input with '#' representing "walls" where the lightray dies.
    aoc_lib::pad_input(input, '#')
        .iter()
        .map(|row| {
            row.iter()
                .map(|c| Tile {
                    energizations: 0,
                    tile_type: match c {
                        '#' => TileType::Wall,
                        '.' => TileType::Empty,
                        '|' => TileType::VerticalSplitter,
                        '-' => TileType::HorizontalSplitter,
                        '/' => TileType::ForwardMirror,
                        '\\' => TileType::BackwardMirror,
                        unknown => unreachable!("Unknown character: {unknown}"),
                    },
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn count_energized(
    mut input: Vec<Vec<Tile>>,
    entry_location: num::Complex<i64>,
    entry_movement_vector: num::Complex<i64>,
) -> usize {
    let mut light_rays = vec![LightRay {
        location: entry_location,
        movement_vector: entry_movement_vector,
    }];

    let mut reflected_light_rays = Vec::new();
    let mut cycle_detection = HashSet::new();

    while !light_rays.is_empty() {
        // Should be fine to go from back. Otherwise we can switch to a VecDeque
        // to be able to use `pop_front`.

        // Interact with the current tile
        for ray in light_rays.drain(..) {
            let re = ray.location.re as usize;
            let im = ray.location.im as usize;
            reflected_light_rays
                .append(&mut ray.interact(&mut input[im][re], &mut cycle_detection));
        }

        // Add the resulting rays from interacting with the tile
        light_rays.append(&mut reflected_light_rays);

        // Step all lightrays to next location.
        for ray in &mut light_rays {
            ray.location += ray.movement_vector;
        }
    }

    input
        .iter()
        .map(|row| row.iter().filter(|tile| tile.energizations > 0).count())
        .sum()
}

fn one(input: Vec<Vec<Tile>>) {
    let now = std::time::Instant::now();

    let energized = count_energized(input, ENTRY_LOCATION, ENTRY_MOVEMENT_VECTOR);

    println!("One: {energized} | Elapsed: {:?}", now.elapsed());
}

/// Brute force potential optimizations:
///
/// 1. Use threads = Brute force faster
/// 2. Be smarter when saving results rather than cloning the input each time.
/// 3. Keep track of direction and result from tiles to shortcircuit calculation.
fn two(input: Vec<Vec<Tile>>) {
    let now = std::time::Instant::now();

    let mut max_energized = 0;

    // Down
    for re in 1..input[1].len() - 1 {
        max_energized = max_energized.max(count_energized(
            input.clone(),
            num::Complex::new(re as i64, 1),
            DOWN,
        ));
    }

    // Up
    for re in 1..input[1].len() - 1 {
        let im = input.len() - 2;
        max_energized = max_energized.max(count_energized(
            input.clone(),
            num::Complex::new(re as i64, im as i64),
            UP,
        ));
    }

    // RIGHT
    for im in 1..input.len() - 1 {
        max_energized = max_energized.max(count_energized(
            input.clone(),
            num::Complex::new(1, im as i64),
            RIGHT,
        ));
    }

    // LEFT
    for im in 1..input.len() - 1 {
        let re = input[1].len() - 2;
        max_energized = max_energized.max(count_energized(
            input.clone(),
            num::Complex::new(re as i64, im as i64),
            RIGHT,
        ));
    }

    println!("Two: {max_energized} | Elapsed: {:?}", now.elapsed());
}

fn main() {
    let input = input();
    one(input.clone());
    two(input);
}

#[cfg(test)]
mod tests {
    use super::*;

    // Positive direction ↓ and →
    // [0,0], [0, 1]
    // [1,0], [1, 1]

    /// Tests '/'
    #[test]
    fn test_forward_reflection() {
        // → /
        assert_eq!(
            num::Complex::new(1, 0) * FORWARD_MIRROR_REFLECTION,
            num::Complex::new(0, -1)
        );
        // / ←
        assert_eq!(
            num::Complex::new(-1, 0) * FORWARD_MIRROR_REFLECTION,
            num::Complex::new(0, 1)
        );

        // /
        // ↑
        assert_eq!(
            num::Complex::new(0, -1) * -FORWARD_MIRROR_REFLECTION,
            num::Complex::new(1, 0)
        );

        // ↓
        // /
        assert_eq!(
            num::Complex::new(0, 1) * -FORWARD_MIRROR_REFLECTION,
            num::Complex::new(-1, 0)
        );
    }

    // Positive direction ↓ and →
    // [0,0], [0, 1]
    // [1,0], [1, 1]

    #[test]
    fn test_backward_reflection() {
        // → \
        assert_eq!(
            num::Complex::new(1, 0) * BACKWARD_MIRROR_REFLECTION,
            num::Complex::new(0, 1)
        );
        // \ ←
        assert_eq!(
            num::Complex::new(-1, 0) * BACKWARD_MIRROR_REFLECTION,
            num::Complex::new(0, -1)
        );

        // \
        // ↑
        assert_eq!(
            num::Complex::new(0, -1) * -BACKWARD_MIRROR_REFLECTION,
            num::Complex::new(-1, 0)
        );

        // ↓
        // \
        assert_eq!(
            num::Complex::new(0, 1) * -BACKWARD_MIRROR_REFLECTION,
            num::Complex::new(1, 0)
        );
    }
}
