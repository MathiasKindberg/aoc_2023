//! Part 1:
//! Part 2:
//!
//! To make it more efficient:
//! 1. Stop using owned values.

use std::{
    collections::{HashMap, VecDeque},
    io::BufRead,
};

type Input = HashMap<String, Module>;
type SignalQueue = VecDeque<Signal>;

fn input() -> Input {
    let stdin = std::io::stdin();
    stdin
        .lock()
        .lines()
        .map_while(Result::ok)
        .map(|row| {
            let (identifier, destinations) = row.split_once(" -> ").unwrap();
            let destinations = destinations
                .split(", ")
                .map(|destination| destination.to_owned())
                .collect();

            if identifier == "broadcaster" {
                (
                    identifier.to_owned(),
                    Module::Broadcaster(Broadcaster {
                        name: identifier.to_owned(),
                        destinations,
                    }),
                )
            } else if &identifier[0..1] == "%" {
                (
                    identifier[1..].to_owned(),
                    Module::FlipFlop(FlipFlop {
                        name: identifier[1..].to_owned(),
                        state: FlipFlopState::Off,
                        destinations,
                    }),
                )
            } else if &identifier[0..1] == "&" {
                (
                    identifier[1..].to_owned(),
                    Module::Conjunction(Conjunction {
                        name: identifier[1..].to_owned(),
                        input_state: HashMap::new(),
                        destinations,
                    }),
                )
            } else {
                unreachable!("Unknown module type: {identifier} from: {row}")
            }
        })
        .collect()
}

#[derive(Debug, Clone)]
struct Signal {
    strength: PulseStrength,
    from: String,
    to: String,
}

impl std::fmt::Display for Signal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.strength {
            PulseStrength::High => write!(f, "{} -high-> {}", self.from, self.to),
            PulseStrength::Low => write!(f, "{} -low-> {}", self.from, self.to),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PulseStrength {
    High,
    Low,
}

#[derive(Debug, Clone)]
enum Module {
    Broadcaster(Broadcaster),
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
}

// Enum dispatch or a trait would be nicer but yeah, this is AoC.
impl Module {
    fn pulse(&mut self, signal: Signal) -> SignalQueue {
        match self {
            Module::Broadcaster(v) => v.pulse(signal),
            Module::FlipFlop(v) => v.pulse(signal),
            Module::Conjunction(v) => v.pulse(signal),
        }
    }

    fn destination_contains_module(&self, module: &String) -> bool {
        match self {
            Module::Broadcaster(v) => v.destinations.contains(module),
            Module::FlipFlop(v) => v.destinations.contains(module),
            Module::Conjunction(v) => v.destinations.contains(module),
        }
    }
}

#[derive(Debug, Clone)]
struct Broadcaster {
    name: String,
    destinations: Vec<String>,
}

impl Broadcaster {
    fn pulse(&self, signal: Signal) -> SignalQueue {
        self.destinations
            .iter()
            .map(|destination| Signal {
                from: self.name.clone(),
                strength: signal.strength,
                to: destination.to_owned(),
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
struct FlipFlop {
    name: String,
    state: FlipFlopState,
    destinations: Vec<String>,
}

impl FlipFlop {
    fn pulse(&mut self, pulse: Signal) -> SignalQueue {
        match pulse.strength {
            // Optimization, return an Option to not have to create an empty queue here.
            PulseStrength::High => std::collections::VecDeque::new(),
            PulseStrength::Low => {
                let strength = match self.state {
                    // Turn on and send a high pulse
                    FlipFlopState::Off => {
                        self.state = FlipFlopState::On;
                        PulseStrength::High
                    }
                    // Turn off and send a low pulse
                    FlipFlopState::On => {
                        self.state = FlipFlopState::Off;
                        PulseStrength::Low
                    }
                };
                self.destinations
                    .iter()
                    .map(|destination| Signal {
                        from: self.name.clone(),
                        strength,
                        to: destination.to_owned(),
                    })
                    .collect()
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum FlipFlopState {
    On,
    Off,
}

#[derive(Debug, Clone)]
struct Conjunction {
    name: String,
    input_state: HashMap<String, PulseStrength>,
    destinations: Vec<String>,
}

impl Conjunction {
    fn pulse(&mut self, signal: Signal) -> SignalQueue {
        self.input_state
            .entry(signal.from)
            .and_modify(|strength| *strength = signal.strength);

        let strength = if self
            .input_state
            .values()
            .all(|strength| strength == &PulseStrength::High)
        {
            PulseStrength::Low
        } else {
            PulseStrength::High
        };

        self.destinations
            .iter()
            .map(|destination| Signal {
                from: self.name.clone(),
                strength,
                to: destination.to_owned(),
            })
            .collect()
    }
}

fn one(input: &Input) {
    let now = std::time::Instant::now();
    let mut low_pulses = 0;
    let mut high_pulses = 0;

    let mut input = input.clone();

    // Connect inputs to conjunction modules.
    let conjunction_modules: Vec<_> = input
        .iter()
        .filter_map(|(key, value)| match value {
            Module::Conjunction(_) => Some(key.clone()),
            _ => None,
        })
        .collect();

    for conjunction in conjunction_modules {
        let connected_modules: Vec<_> = input
            .iter()
            .filter_map(|(name, module)| {
                if module.destination_contains_module(&conjunction) {
                    Some(name.clone())
                } else {
                    None
                }
            })
            .collect();

        match input.get_mut(&conjunction).unwrap() {
            Module::Conjunction(conjunction) => conjunction.input_state.extend(
                connected_modules
                    .iter()
                    .map(|input_module| (input_module.to_owned(), PulseStrength::Low)),
            ),
            not_conjunction => unreachable!("Should never happen.... Got: {not_conjunction:?}"),
        }
    }

    // VecDeque since we want to pull pulses from the top and push new ones to the end.
    let mut pulse_queue: SignalQueue = VecDeque::new();
    for _ in 1..=1000 {
        assert!(pulse_queue.is_empty());

        // Button push
        pulse_queue.push_back(Signal {
            strength: PulseStrength::Low,
            from: String::from("button"),
            to: String::from("broadcaster"),
        });

        while let Some(pulse) = pulse_queue.pop_front() {
            // println!("{pulse}");
            match pulse.strength {
                PulseStrength::High => high_pulses += 1,
                PulseStrength::Low => low_pulses += 1,
            }

            if let Some(module) = input.get_mut(&pulse.to) {
                let mut resulting_pulses = module.pulse(pulse);
                pulse_queue.append(&mut resulting_pulses);
            }

            // Pulses are always processed in the order they are
            // sent. So, if a pulse is sent to modules a, b, and c,
            // and then module a processes its pulse and sends more
            // pulses, the pulses sent to modules b and c would
            // have to be handled first.

            // I.e. breadth first.
        }
        // println!("------- Button push: {idx} High pulses: {high_pulses}: Low pulses: {low_pulses} -------\n")
    }

    let sum = high_pulses * low_pulses;
    println!("One: {sum} | Elapsed: {:?}", now.elapsed());
}

fn two(_input: &Input) {
    let now = std::time::Instant::now();
    let sum = 0;

    println!("Two: {sum} | Elapsed: {:?}", now.elapsed());
}

fn main() {
    let input = input();
    one(&input);
    two(&input);
}
