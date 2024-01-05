//! Part 1:
//! Part 2:

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
                unreachable!("Unknown module type")
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

#[derive(Debug, Clone, Copy)]
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

impl Module {
    fn pulse(&mut self, signal: Signal) -> SignalQueue {
        match self {
            Module::Broadcaster(v) => v.pulse(signal),
            Module::FlipFlop(v) => v.pulse(signal),
            Module::Conjunction(v) => v.pulse(signal),
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
    fn pulse(&self, signal: Signal) -> SignalQueue {
        todo!("Flippy floppy!")
    }
}

#[derive(Debug, Clone)]
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
        // Assume state is low if doesn't exist. Otherwise update state.
        self.input_state.entry(signal.from);

        todo!("conjunction!")
    }
}

fn one(input: &Input) {
    let now = std::time::Instant::now();
    let sum = 0;
    let mut input = input.clone();

    for row in &input {
        println!("{row:?}");
    }

    // VecDeque since we want to pull pulses from the top and push new ones to the end.
    let mut pulse_queue: VecDeque<Signal> = input
        .get_mut("broadcaster")
        .unwrap()
        // Start with sending initial signal (later in a loop I think.)
        .pulse(Signal {
            strength: PulseStrength::Low,
            from: String::from("button"),
            to: String::from("broadcaster"),
        });

    println!("\n\nPulse queue: {pulse_queue:?}");

    while let Some(pulse) = pulse_queue.pop_front() {
        println!("\n----------------\n{pulse:?}");

        let resulting_pulses = input.get_mut(&pulse.to).unwrap().pulse(pulse);
        // pulse_queue.append(&mut result);
        println!("{resulting_pulses:?}");
        println!("Pulse queue: {pulse_queue:?}");

        // Pulses are always processed in the order they are
        // sent. So, if a pulse is sent to modules a, b, and c,
        // and then module a processes its pulse and sends more
        // pulses, the pulses sent to modules b and c would
        // have to be handled first.

        // I.e. breadth first.
    }

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
