use anyhow::Result;
use itertools::Itertools;
use lazy_regex::regex_captures;
use std::collections::HashMap;
use std::collections::VecDeque;
use PulseType::*;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
enum PulseType {
    High,
    Low,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum ModuleType {
    Broadcaster,
    Conjunction {
        input_states: HashMap<String, PulseType>,
    },
    FlipFlop {
        on: bool,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Module {
    recipients: Vec<String>,
    name: String,
    module_type: ModuleType,
}

#[aoc::main]
fn solve(input: &str) -> Result<usize> {
    let mut modules: HashMap<String, Module> = HashMap::new();

    aoc::parse_list::<String>(input)?.iter().for_each(|line| {
        let (_, prefix, name, destinations) =
            regex_captures!(r#"([%&])?([a-z]+) -> (.+)$"#, line).unwrap();

        let destinations = destinations
            .trim()
            .split(", ")
            .map(|s| s.to_string())
            .collect_vec();

        let name = name.to_string();

        if prefix.is_empty() && name == "broadcaster" {
            modules.insert(
                name.clone(),
                Module {
                    name,
                    recipients: destinations,
                    module_type: ModuleType::Broadcaster,
                },
            );
        } else if prefix == "&" {
            modules.insert(
                name.clone(),
                Module {
                    name,
                    recipients: destinations,
                    module_type: ModuleType::Conjunction {
                        input_states: HashMap::new(),
                    },
                },
            );
        } else if prefix == "%" {
            modules.insert(
                name.clone(),
                Module {
                    name,
                    recipients: destinations,
                    module_type: ModuleType::FlipFlop { on: false },
                },
            );
        } else {
            panic!("Unexpected prefix, {prefix}");
        }
    });

    // Add destinations to all conjunctions
    let mut updates = Vec::new();
    modules.values().for_each(|m| {
        for dest in &m.recipients {
            if let Some(Module {
                recipients: _,
                name: _,
                module_type: ModuleType::Conjunction { input_states: _ },
            }) = &modules.get(dest)
            {
                updates.push((dest.to_string(), m.name.to_string()));
            }
        }
    });
    for update in updates {
        let m = modules.get_mut(&update.0).unwrap();
        if let ModuleType::Conjunction { input_states: hm } = &mut m.module_type {
            hm.insert(update.1, Low);
        }
    }

    let mut low_pulses = 0;
    let mut high_pulses = 0;
    for i in 0..1000 {
        let (h, l) = perform_button_press(&mut modules);
        low_pulses += l;
        high_pulses += h;
    }

    println!("{modules:?}");

    let answer = low_pulses * high_pulses;

    Ok(answer)
}

fn perform_button_press(modules: &mut HashMap<String, Module>) -> (usize, usize) {
    let mut lp = 0;
    let mut hp = 0;

    let mut deque: VecDeque<Vec<_>> = VecDeque::new();
    deque.push_back(vec![("button".to_string(), "broadcaster".to_string(), Low)]);
    while let Some(generation) = deque.pop_front() {
        println!();
        println!();
        println!("Start Generation");
        let mut new_generation = Vec::new();
        for (source, dest, pulse) in generation {
            // Increment the pulse counts
            match pulse {
                High => hp += 1,
                Low => lp += 1,
            };

            println!("{dest} received {pulse:?}");

            if let Some(m) = modules.get_mut(&dest) {
                match m.module_type {
                    ModuleType::Broadcaster => {
                        new_generation.extend(
                            m.recipients
                                .iter()
                                .map(|name| (m.name.to_string(), name.to_string(), pulse.clone()))
                                .collect_vec(),
                        );
                    }
                    ModuleType::FlipFlop { on: ref mut is_on } => {
                        if matches!(pulse, Low) {
                            *is_on = !*is_on;
                            let new_pulse = if *is_on { High } else { Low };
                            new_generation.extend(
                                m.recipients
                                    .iter()
                                    .map(|name| {
                                        (m.name.to_string(), name.to_string(), new_pulse.clone())
                                    })
                                    .collect_vec(),
                            );
                        }
                    }
                    ModuleType::Conjunction {
                        input_states: ref mut inputs,
                    } => {
                        inputs.insert(source.to_string(), pulse.clone());
                        println!("{inputs:?}");
                        let new_pulse = if inputs.values().all(|p| *p == High) {
                            Low
                        } else {
                            High
                        };
                        new_generation.extend(
                            m.recipients
                                .iter()
                                .map(|name| {
                                    (m.name.to_string(), name.to_string(), new_pulse.clone())
                                })
                                .collect_vec(),
                        );
                    }
                }
            }
        }
        if !new_generation.is_empty() {
            deque.push_back(new_generation);
        }
    }

    (hp, lp)
}

fn tests() -> anyhow::Result<()> {
    let input = r"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
";

    let solution = solve(input)?;

    assert_eq!(solution, 32000000);

    println!("Test 1 passed");
    let input = r"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
";

    let solution = solve(input)?;

    assert_eq!(solution, 11687500);
    println!("Test 2 passed");

    Ok(())
}
