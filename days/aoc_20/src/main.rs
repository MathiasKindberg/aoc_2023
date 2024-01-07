//! Part 1: 808146535
//! Part 2:
//!
//! To make it more efficient:
//! 1. Stop using owned values.

use std::{
    collections::{HashMap, VecDeque},
    io::BufRead,
};

type Input<'a> = (ModuleMap<'a>, DestinationMap<'a>);
type ModuleMap<'a> = HashMap<&'a str, Module<'a>>;
type DestinationMap<'a> = HashMap<&'a str, Vec<&'a str>>;
type SignalQueue<'a> = VecDeque<Signal<'a>>;

fn input() -> Vec<String> {
    let stdin = std::io::stdin();
    stdin.lock().lines().map_while(Result::ok).collect()
}

#[derive(Debug, Clone)]
struct Signal<'a> {
    strength: PulseStrength,
    from: &'a str,
    to: &'a str,
}

impl std::fmt::Display for Signal<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.strength {
            PulseStrength::High => write!(f, "{} -high-> {}", self.from, self.to),
            PulseStrength::Low => write!(f, "{} -low-> {}", self.from, self.to),
        }
    }
}

impl<'a> Signal<'a> {
    fn send(
        self,
        module_map: &mut ModuleMap<'a>,
        destination_map: &DestinationMap<'a>,
        queue: &mut SignalQueue<'a>,
    ) {
        let Some(module) = module_map.get_mut(self.to) else {
            return;
        };

        let send = match module {
            Module::Broadcaster => Some(self.strength),
            Module::FlipFlop { state } => match self.strength {
                PulseStrength::High => None,
                PulseStrength::Low => {
                    match state {
                        // Turn on and send a high pulse
                        FlipFlopState::Off => {
                            *state = FlipFlopState::On;
                            Some(PulseStrength::High)
                        }
                        // Turn off and send a low pulse
                        FlipFlopState::On => {
                            *state = FlipFlopState::Off;
                            Some(PulseStrength::Low)
                        }
                    }
                }
            },
            Module::Conjunction { memory } => {
                let input = memory.get_mut(self.from).unwrap();
                *input = self.strength;

                if memory
                    .values()
                    .all(|strength| strength == &PulseStrength::High)
                {
                    Some(PulseStrength::Low)
                } else {
                    Some(PulseStrength::High)
                }
            }
        };
        if let Some(strength) = send {
            for destination in destination_map.get(self.to).unwrap() {
                queue.push_back(Signal {
                    from: self.to,
                    strength,
                    to: destination,
                })
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PulseStrength {
    High,
    Low,
}

#[derive(Debug, Clone)]
enum Module<'a> {
    Broadcaster,
    FlipFlop {
        state: FlipFlopState,
    },
    Conjunction {
        memory: HashMap<&'a str, PulseStrength>,
    },
}

#[derive(Debug, Clone, Copy)]
enum FlipFlopState {
    On,
    Off,
}

fn one(input: &Input) {
    let now = std::time::Instant::now();
    let mut low_pulses = 0;
    let mut high_pulses = 0;

    let (mut module_map, destination_map) = input.clone();

    // VecDeque since we want to pull pulses from the top and push new ones to the end.
    let mut pulse_queue: SignalQueue = VecDeque::new();
    for _ in 1..=1000 {
        // Button push
        pulse_queue.push_back(Signal {
            strength: PulseStrength::Low,
            from: "button",
            to: "broadcaster",
        });

        while let Some(pulse) = pulse_queue.pop_front() {
            match pulse.strength {
                PulseStrength::High => high_pulses += 1,
                PulseStrength::Low => low_pulses += 1,
            }
            pulse.send(&mut module_map, &destination_map, &mut pulse_queue);
        }
    }

    let sum = high_pulses * low_pulses;
    println!("One: {sum} | Elapsed: {:?}", now.elapsed());
}

// /// Idea:
// /// 1. Find the LCM for all inputs to RG.
// fn two(input: &Input) {
//     let now = std::time::Instant::now();
//     let mut low_pulses = 0;
//     let mut high_pulses = 0;
//     let mut input = input.clone();

//     // Connect inputs to conjunction modules.
//     let conjunction_modules: Vec<_> = input
//         .iter()
//         .filter_map(|(key, value)| match value {
//             Module::Conjunction(_) => Some(key.clone()),
//             _ => None,
//         })
//         .collect();

//     for conjunction in conjunction_modules {
//         let connected_modules: Vec<_> = input
//             .iter()
//             .filter_map(|(name, module)| {
//                 if module.destination_contains_module(&conjunction) {
//                     Some(name.clone())
//                 } else {
//                     None
//                 }
//             })
//             .collect();

//         match input.get_mut(&conjunction).unwrap() {
//             Module::Conjunction(conjunction) => conjunction.input_state.extend(
//                 connected_modules
//                     .iter()
//                     .map(|input_module| (input_module.to_owned(), PulseStrength::Low)),
//             ),
//             not_conjunction => unreachable!("Should never happen.... Got: {not_conjunction:?}"),
//         }
//     }

//     let mut pulse_queue: SignalQueue = VecDeque::new();

//     // We need to find when all inputs for "rg" is high.
//     // Or rather, we need to find multipliers for all conjunctions....
//     for button_presses in 1..=10000000 {
//         // Button push
//         pulse_queue.push_back(Signal {
//             strength: PulseStrength::Low,
//             from: "button",
//             to: "broadcaster",
//         });

//         while let Some(pulse) = pulse_queue.pop_front() {
//             match pulse.strength {
//                 PulseStrength::High => high_pulses += 1,
//                 PulseStrength::Low => low_pulses += 1,
//             }

//             if let Some(module) = input.get_mut(pulse.to) {
//                 let mut resulting_pulses = module.pulse(pulse);
//                 pulse_queue.append(&mut resulting_pulses);
//             }
//         }

//         // let mut high = false;
//         // let conj = input.get("gs").unwrap().as_conjunction();
//         // for (_, value) in &conj.input_state {
//         //     if value == &PulseStrength::Low {
//         //         high = true;
//         //     }
//         // }

//         // if high {
//         //     println!("{button_presses}: {:?}", conj.input_state)
//         // }

//         // if low_pulses_to_rx != 0 {
//         //     println!("Low to rx: {low_pulses_to_rx} after presses: {button_presses}");
//         // }
//         // if low_pulses_to_rx == 1 {
//         //     println!("Two: {button_presses} | Elapsed: {:?}", now.elapsed());
//         // }
//     }
//     println!("Two: | Elapsed: {:?}", now.elapsed());
// }

fn parse(input: &Vec<String>) -> Input {
    let mut module_map = HashMap::new();
    let mut destination_map = HashMap::new();

    // Parse input and build graphs
    for row in input {
        let (identifier, destinations) = row.split_once(" -> ").unwrap();
        let destinations = destinations.split(", ").collect();

        let (identifier, module) = if identifier == "broadcaster" {
            (identifier, Module::Broadcaster)
        } else if &identifier[0..1] == "%" {
            (
                &identifier[1..],
                Module::FlipFlop {
                    state: FlipFlopState::Off,
                },
            )
        } else if &identifier[0..1] == "&" {
            (
                &identifier[1..],
                Module::Conjunction {
                    memory: HashMap::new(),
                },
            )
        } else {
            unreachable!("Unknown module type: {identifier} from: {row}")
        };

        module_map.insert(identifier, module);
        destination_map.insert(identifier, destinations);
    }

    // Set all inputs to low for conjunction
    for (source, destinations) in &destination_map {
        for destination in destinations {
            if let Some(Module::Conjunction { memory }) = module_map.get_mut(destination) {
                memory.insert(source, PulseStrength::Low);
            }
        }
    }

    (module_map, destination_map)
}

fn main() {
    let input = input();
    let input = parse(&input);

    one(&input);
    // two(&input);
}
