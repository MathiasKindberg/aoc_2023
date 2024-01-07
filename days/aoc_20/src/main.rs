//! Part 1: 808146535
//! Part 2:

use core::panic;
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

// Find LCM of inputs to rg.
fn two(input: &Input) {
    let now = std::time::Instant::now();

    let (mut module_map, destination_map) = input.clone();

    let (module_before_rx, _) = destination_map
        .iter()
        .find(|(_, destinations)| destinations.contains(&"rx"))
        .unwrap();

    // VecDeque since we want to pull pulses from the top and push new ones to the end.
    let mut pulse_queue: SignalQueue = VecDeque::new();

    let Module::Conjunction { memory } = module_map.get(module_before_rx).unwrap() else {
        panic!()
    };

    let mut lcm_tracker: HashMap<&str, Option<usize>> =
        memory.keys().map(|module| (*module, None)).collect();

    'button_press_loop: for butten_presses in 1..=100000000 {
        // Button push
        assert!(pulse_queue.is_empty());
        pulse_queue.push_back(Signal {
            strength: PulseStrength::Low,
            from: "button",
            to: "broadcaster",
        });

        while let Some(pulse) = pulse_queue.pop_front() {
            if pulse.to == *module_before_rx && pulse.strength == PulseStrength::High {
                *lcm_tracker.get_mut(pulse.from).unwrap() = Some(butten_presses);

                if lcm_tracker.values().all(|presses| presses.is_some()) {
                    let lcm = lcm_tracker
                        .iter()
                        .fold(None, |mut lcm_acc, (_, button_presses)| {
                            let button_presses = button_presses.unwrap();
                            match lcm_acc {
                                None => lcm_acc = Some(button_presses),
                                Some(curr_lcm) => {
                                    lcm_acc = Some(aoc_lib::lcm(curr_lcm, button_presses))
                                }
                            }
                            lcm_acc
                        })
                        .unwrap();

                    println!("Two: {lcm} | Elapsed: {:?}", now.elapsed());
                    break 'button_press_loop;
                }
            }
            pulse.send(&mut module_map, &destination_map, &mut pulse_queue);
        }
    }
}

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
    two(&input);
}
