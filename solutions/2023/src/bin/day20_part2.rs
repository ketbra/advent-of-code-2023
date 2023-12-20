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

    // // Print out the modules for graphviz
    // for m in modules.values() {
    //     for dest in &m.recipients {
    //         println!("{} -> {};", m.name, dest);
    //     }
    // }
    // panic!("Done");
    let mut answer = 0;
    let interesting_nodes = ["lk", "zv", "xt", "sp"];
    // let conjunctions = ["jc", "vv", "dv", "xq"];

    // // These conjunction inputs are n bit counters, so figure out how big each counter is
    // let mut counter_lengths = Vec::new();
    // for name in &conjunctions {
    //     let m = modules.get(*name).unwrap();
    //     if let Some(Module {
    //         recipients: _,
    //         name: _,
    //         module_type: ModuleType::Conjunction { input_states },
    //     }) = modules.get(*name)
    //     {
    //         println!("{name} has {} inputs", input_states.len());
    //         counter_lengths.push(2_u64.pow(input_states.len() as u32));
    //     }
    // }

    let mut press_count = 0;
    loop {
        answer += 1;
        press_count += 1;
        let rx_count = perform_button_press(&mut modules, press_count);
        if rx_count == 1 {
            break;
        }

        for name in &interesting_nodes {
            let m = modules.get(*name).unwrap();
            // println!("{m:?}");
            if let Some(Module {
                recipients: _,
                name: _,
                module_type: ModuleType::FlipFlop { on, flips: _ },
            }) = modules.get(*name)
            {
                println!("{name} is {on} inputs");
            }
        }

        // print_states(&modules);

        // for name in &conjunctions {
        // for name in &["jc"] {
        //     let m = modules.get(*name).unwrap();
        //     if let Some(Module {
        //         recipients: _,
        //         name: _,
        //         module_type: ModuleType::Conjunction { input_states },
        //     }) = modules.get(*name)
        //     {
        //         println!(
        //             "{name}:{}",
        //             input_states
        //                 .keys()
        //                 .sorted()
        //                 .map(|key| {
        //                     if *input_states.get(key).unwrap() == High {
        //                         '1'
        //                     } else {
        //                         '0'
        //                     }
        //                 })
        //                 .join("")
        //         );
        //         // println!("{name} is on at press {answer}");
        //     }

        //     // for recipient in &m.recipients {}
        //     // if let Some(Module {
        //     //     recipients: _,
        //     //     name: _,
        //     //     module_type: ModuleType::FlipFlop { on: true, flips: _ },
        //     // }) = modules.get(*name)
        //     // {
        //     //     println!("{name} is on at press {answer}");
        //     // }
        // }

        if answer % 100000 == 0 {
            println!("{answer}");
        }
    }

    Ok(answer)
}

fn print_states(modules: &HashMap<String, Module>) {
    // Find all flip flops and print the states
    let mut flipflops = modules
        .values()
        .filter_map(|m| match &m.module_type {
            ModuleType::FlipFlop { on: _, flips } => Some((m, *flips)),
            _ => None,
        })
        .collect_vec();

    flipflops.sort_unstable_by_key(|t| t.1);
    let s = flipflops
        .iter()
        .map(|t| match t.0.module_type {
            ModuleType::FlipFlop { on: true, flips: _ } => '1',
            _ => '0',
        })
        .join("");

    println!("{s}");
}

fn perform_button_press(modules: &mut HashMap<String, Module>, press_count: usize) -> usize {
    let mut lp = 0;
    let mut hp = 0;
    let mut rx_count = 0;

    let mut deque: VecDeque<Vec<_>> = VecDeque::new();
    deque.push_back(vec![("button".to_string(), "broadcaster".to_string(), Low)]);
    while let Some(generation) = deque.pop_front() {
        // println!();
        // println!();
        // println!("Start Generation");
        let mut new_generation = Vec::new();
        for (source, dest, pulse) in generation {
            if dest == "dg" && pulse == High {
                println!("{press_count}: dg received a High from {source}");
            }

            // Increment the pulse counts
            match pulse {
                High => hp += 1,
                Low => lp += 1,
            };

            // println!("{dest} received {pulse:?}");

            if dest == "rx" && matches!(pulse, Low) {
                rx_count += 1;
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
                        // println!("{inputs:?}");
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

    rx_count
}

fn tests() -> anyhow::Result<()> {
    //     let input = r"broadcaster -> a, b, c
    // %a -> b
    // %b -> c
    // %c -> inv
    // &inv -> a
    // ";

    //     let solution = solve(input)?;

    //     assert_eq!(solution, 32000000);

    //     println!("Test 1 passed");
    //     let input = r"broadcaster -> a
    // %a -> inv, con
    // &inv -> b
    // %b -> con
    // &con -> output
    // ";

    //     let solution = solve(input)?;

    //     assert_eq!(solution, 11687500);
    //     println!("Test 2 passed");

    Ok(())
}
