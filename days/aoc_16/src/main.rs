//! Part 1:
//! Part 2:

use std::io::BufRead;

#[derive(Debug, Clone, Copy)]
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
    fn interact(mut self, tile: &mut Tile) -> Vec<LightRay> {
        use num::complex::ComplexFloat;

        tile.energizations += 1;

        match tile.tile_type {
            // Simple way to ensure light rays leaving the 2d space
            // dies.
            TileType::Wall => {
                println!("WALL");
                vec![]
            }

            TileType::Empty => vec![self],

            TileType::ForwardMirror => todo!("FORWARD"),
            TileType::BackwardMirror => {
                println!("BACKWARD MIRROR");
                println!("{self:?}");
                println!("{tile:?}");
                todo!("BACKWARD")
            }

            TileType::VerticalSplitter => {
                println!("VERTICAL SPLIT: {}", self.location);

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
                println!("HORIZONTAL SPLIT: {}", self.location);
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
                    assert_eq!(resulting_rays.len(), 2);
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

fn one(mut input: Vec<Vec<Tile>>) {
    let now = std::time::Instant::now();
    let sum = 0;

    for row in &input {
        for tile in row {
            print!("{}", tile.tile_type);
        }
        println!()
    }

    let mut light_rays = vec![LightRay {
        location: ENTRY_LOCATION,
        movement_vector: ENTRY_MOVEMENT_VECTOR,
    }];

    let mut reflected_light_rays = Vec::new();

    while !light_rays.is_empty() {
        // Should be fine to go from back. Otherwise we can switch to a VecDeque
        // to be able to use `pop_front`.

        // Interact with the current tile
        for ray in light_rays.drain(..) {
            let re = ray.location.re as usize;
            let im = ray.location.im as usize;
            println!("Interacting: {re}, {im}, {}", input[im][re].tile_type);
            reflected_light_rays.append(&mut ray.interact(&mut input[im][re]));
        }

        // Add the resulting rays from interacting with the tile
        light_rays.append(&mut reflected_light_rays);

        // Step all lightrays to next location.
        for ray in &mut light_rays {
            println!("--------------------");
            println!("FROM: {}", ray.location);
            ray.location = ray.location + ray.movement_vector;
            println!("TO: {}", ray.location);
        }

        println!("\n===== STEP DONE ====")
    }

    println!("One: {sum} | Elapsed: {:?}", now.elapsed());
}

fn two(_input: &[Vec<Tile>]) {
    let now = std::time::Instant::now();
    let sum = 0;

    println!("Two: {sum} | Elapsed: {:?}", now.elapsed());
}

fn main() {
    let input = input();
    one(input.clone());
    two(&input);
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
