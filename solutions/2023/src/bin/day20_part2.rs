use anyhow::Result;
use itertools::Itertools;
use lazy_regex::regex_captures;
use std::collections::HashMap;
use std::collections::HashSet;
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
        flips: usize,
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
                    module_type: ModuleType::FlipFlop {
                        on: false,
                        flips: 0,
                    },
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

    let rx_trigger = modules
        .values()
        .find(|m| m.recipients.contains(&"rx".to_string()))
        .unwrap();

    // RX is triggered by a conjunction.  We will find the input nodes
    // to that conjunction and determine the periods for those input
    // nodes.  Those input nodes are the output of counter counter
    // circuits each with a different period.  When they all trigger
    // concurrently, then the rx_trigger conjunction triggers and
    // subsequently rx does as well.
    let mut interesting_nodes = HashSet::new();
    if let ModuleType::Conjunction { input_states } = &rx_trigger.module_type {
        interesting_nodes = input_states
            .keys()
            .map(|x| x.to_string())
            .collect::<HashSet<_>>();
    }

    println!("Counter outputs: {interesting_nodes:?}");

    let mut press_count = 0;
    let mut periods = Vec::new();
    loop {
        press_count += 1;
        let triggered_nodes = perform_button_press(&mut modules, &interesting_nodes);
        for name in triggered_nodes {
            periods.push(press_count);
            interesting_nodes.remove(&name);
        }

        // If we've found all of the periods, then stop searching
        if interesting_nodes.is_empty() {
            break;
        }
    }

    println!("Counter periods: {periods:?}");

    // The periods appear to be prime in my data, so just multiplying
    // them should be the answer.  Still, the lcm could be used for
    // robustnuess in case the numbers aren't at least coprime.  For
    // additional robustness we could verify that we weren't partially
    // through a counter cycle for our initial condition.  Again, this
    // seems unnecessary given the actual input file I received, but
    // it could be added for robustness.

    let answer = periods.iter().product();

    Ok(answer)
}

fn perform_button_press(
    modules: &mut HashMap<String, Module>,
    interesting_nodes: &HashSet<String>,
) -> HashSet<String> {
    let mut triggered_interesting_nodes = HashSet::new();
    let mut deque: VecDeque<Vec<_>> = VecDeque::new();
    deque.push_back(vec![("button".to_string(), "broadcaster".to_string(), Low)]);
    while let Some(generation) = deque.pop_front() {
        let mut new_generation = Vec::new();
        for (source, dest, pulse) in generation {
            if interesting_nodes.contains(&dest) && pulse == Low {
                triggered_interesting_nodes.insert(dest.to_string());
            }
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
                    ModuleType::FlipFlop {
                        on: ref mut is_on,
                        ref mut flips,
                    } => {
                        if matches!(pulse, Low) {
                            *is_on = !*is_on;
                            *flips += 1;
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
    triggered_interesting_nodes
}

fn tests() -> anyhow::Result<()> {
    Ok(())
}
