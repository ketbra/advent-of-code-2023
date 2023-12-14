use anyhow::Result;
use regex::Regex;
use std::cmp;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::env;

#[derive(Clone, Debug)]
struct Blueprint {
    ordinal: u16,
    ore_ore: u16,
    clay_ore: u16,
    obsidian_ore: u16,
    obsidian_clay: u16,
    geode_ore: u16,
    geode_obsidian: u16,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct State {
    minutes_left: u16,
    ore: u16,
    clay: u16,
    obsidian: u16,
    geodes: u16,
    ore_robots: u16,
    clay_robots: u16,
    obsidian_robots: u16,
    geode_robots: u16,
}

fn solve_blueprint(blueprint: &Blueprint) -> u16 {
    let mut deque: VecDeque<State> = VecDeque::new();
    deque.push_back(State {
        minutes_left: 32,
        ore: 0,
        clay: 0,
        obsidian: 0,
        geodes: 0,
        ore_robots: 1,
        clay_robots: 0,
        obsidian_robots: 0,
        geode_robots: 0,
    });

    let mut max: u16 = 0;
    // let cache: HashMap<State, u16> = HashMap::new();
    let mut seen: HashSet<State> = HashSet::new();
    let max_ore_robots_needed = cmp::max(
        blueprint.ore_ore,
        cmp::max(
            blueprint.obsidian_ore,
            cmp::max(blueprint.clay_ore, blueprint.geode_ore),
        ),
    );
    let max_clay_robots_needed = blueprint.obsidian_clay;
    let max_obsidian_robots_needed = blueprint.geode_obsidian;

    while let Some(state) = deque.pop_front() {
        // println!("{state:?}");

        if state.minutes_left == 0 {
            if state.geodes > max {
                max = state.geodes;
                // println!("Current max: {max}");
                // println!("Current max: {state:?}");
            }
        } else if !seen.contains(&state) {
            seen.insert(state.clone());

            let should_build_ore_robot = state.ore_robots < max_ore_robots_needed;
            let should_build_clay_robot = state.clay_robots < max_clay_robots_needed;
            let should_build_obsidian_robot =
                state.clay_robots > 0 && state.obsidian_robots < max_obsidian_robots_needed;
            let should_build_geode_robot = state.obsidian_robots > 0;

            // There is no point in creating multiple paths that are
            // eventually going to create a particular robot next.  If
            // we want to create a particular robot next, then just
            // move ahead as efficiently as possible to that state.
            // without adding the inbetween states.  There is no point
            // in taking a less efficient path that creates the same
            // robot next

            if should_build_geode_robot {
                // Determine how many minutes we need to wait until we can build one
                let ore_delay = if state.ore < blueprint.geode_ore {
                    div_ceil(blueprint.geode_ore - state.ore, state.ore_robots)
                } else {
                    0
                };
                let obsidian_delay = if state.obsidian < blueprint.geode_obsidian {
                    div_ceil(
                        blueprint.geode_obsidian - state.obsidian,
                        state.obsidian_robots,
                    )
                } else {
                    0
                };
                let delay = cmp::max(ore_delay, obsidian_delay) + 1;

                if delay < state.minutes_left {
                    let mut state_ = state.clone();
                    state_.geode_robots += 1;
                    state_.minutes_left = state_.minutes_left - delay;
                    state_.obsidian = state_.obsidian + (delay * state.obsidian_robots)
                        - blueprint.geode_obsidian;
                    state_.ore = state_.ore + (delay * state.ore_robots) - blueprint.geode_ore;
                    state_.clay = state_.clay + (delay * state.clay_robots);
                    state_.geodes = state_.geodes + (delay * state.geode_robots);
                    deque.push_back(state_);
                }
            }

            if should_build_obsidian_robot {
                // Determine how many minutes we need to wait until we can build one
                let ore_delay = if state.ore < blueprint.obsidian_ore {
                    div_ceil(blueprint.obsidian_ore - state.ore, state.ore_robots)
                } else {
                    0
                };
                let clay_delay = if state.clay < blueprint.obsidian_clay {
                    div_ceil(blueprint.obsidian_clay - state.clay, state.clay_robots)
                } else {
                    0
                };
                let delay = cmp::max(ore_delay, clay_delay) + 1;

                if delay < state.minutes_left {
                    let mut state_ = state.clone();
                    state_.obsidian_robots += 1;
                    state_.minutes_left = state_.minutes_left - delay;
                    state_.obsidian = state_.obsidian + (delay * state.obsidian_robots);
                    state_.ore = state_.ore + (delay * state.ore_robots) - blueprint.obsidian_ore;
                    state_.clay =
                        state_.clay + (delay * state.clay_robots) - blueprint.obsidian_clay;
                    state_.geodes = state_.geodes + (delay * state.geode_robots);
                    deque.push_back(state_);
                }
            }

            if should_build_clay_robot {
                // println!("Should build clay robot");

                // Determine how many minutes we need to wait until we can build one
                let delay = 1 + if state.ore < blueprint.clay_ore {
                    div_ceil(blueprint.clay_ore - state.ore, state.ore_robots)
                } else {
                    0
                };

                if delay < state.minutes_left {
                    let mut state_ = state.clone();
                    state_.clay_robots += 1;
                    state_.minutes_left = state_.minutes_left - delay;
                    state_.obsidian = state_.obsidian + (delay * state.obsidian_robots);
                    state_.ore = state_.ore + (delay * state.ore_robots) - blueprint.clay_ore;
                    state_.clay = state_.clay + (delay * state.clay_robots);
                    state_.geodes = state_.geodes + (delay * state.geode_robots);
                    deque.push_back(state_);
                }
            }

            if should_build_ore_robot {
                // println!("Should build ore robot");
                // Determine how many minutes we need to wait until we can build one
                let delay = 1 + if state.ore < blueprint.ore_ore {
                    div_ceil(blueprint.ore_ore - state.ore, state.ore_robots)
                } else {
                    0
                };

                // println!(
                //     "Ore={}, Robots={}, Delay={delay}",
                //     state.ore, state.ore_robots
                // );

                if delay < state.minutes_left {
                    let mut state_ = state.clone();
                    state_.ore_robots += 1;
                    state_.minutes_left = state_.minutes_left - delay;
                    state_.obsidian = state_.obsidian + (delay * state.obsidian_robots);
                    state_.ore = state_.ore + (delay * state.ore_robots) - blueprint.ore_ore;
                    state_.clay = state_.clay + (delay * state.clay_robots);
                    state_.geodes = state_.geodes + (delay * state.geode_robots);
                    deque.push_back(state_);
                }
            }

            // If there aren't enough minutes to build a robot, then
            // add the base state so that we hit the zero condition
            if state.minutes_left < 2 {
                let mut new_base_state = state.clone();
                new_base_state.minutes_left -= 1;
                new_base_state.geodes += state.geode_robots;
                new_base_state.obsidian += state.obsidian_robots;
                new_base_state.clay += state.clay_robots;
                new_base_state.ore += state.ore_robots;
                deque.push_back(new_base_state);
            }
        }
    }

    // println!("{blueprint:?}");
    return max;
}

fn div_ceil(i: u16, j: u16) -> u16 {
    if i % j == 0 {
        return i / j;
    } else {
        return (i / j) + 1;
    }
}

#[aoc::main]
fn solve(input: &str) -> Result<u16> {
    let mut blueprints: Vec<Blueprint> = Vec::new();

    for line in aoc::parse_list::<String>(input)? {
        let caps = Regex::new(r"^Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.$")
            .expect("Regex Error")
            .captures(&line)
            .unwrap();

        blueprints.push(Blueprint {
            ordinal: caps[1].parse::<u16>().unwrap(),
            ore_ore: caps[2].parse::<u16>().unwrap(),
            clay_ore: caps[3].parse::<u16>().unwrap(),
            obsidian_ore: caps[4].parse::<u16>().unwrap(),
            obsidian_clay: caps[5].parse::<u16>().unwrap(),
            geode_ore: caps[6].parse::<u16>().unwrap(),
            geode_obsidian: caps[7].parse::<u16>().unwrap(),
        });
    }

    let mut answer = 1;
    for blueprint in blueprints.into_iter().take(3) {
        // println!("{blueprint:?}");
        let geodes = solve_blueprint(&blueprint);
        println!("Blueprint {} max is {} geodes", blueprint.ordinal, geodes);
        answer *= geodes;
    }
    Ok(answer)
}

fn tests() -> anyhow::Result<()> {
    Ok(())
}
