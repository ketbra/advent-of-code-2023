use anyhow::Result;
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct PositionState {
    position: String,
    step: usize,
}

#[aoc::main]
fn solve(input: &str) -> Result<u64> {
    let lines = aoc::parse_list::<String>(input)?;

    let instructions = lines[0].chars().collect_vec();
    let mut transitions: HashMap<String, HashMap<char, String>> = HashMap::new();
    for line in lines.into_iter().skip(2) {
        let v = line.split('=').collect_vec();
        let key = v[0].trim();
        let mut dests = v[1].split(',').collect_vec();
        dests[0] = &dests[0].trim()[1..];
        dests[1] = &dests[1].trim()[..dests[1].len() - 2];

        transitions.insert(
            key.to_string(),
            HashMap::from([('L', dests[0].to_string()), ('R', dests[1].to_string())]),
        );
    }

    let mut states = transitions
        .keys()
        .filter(|x| x.ends_with('A'))
        .map(|x| PositionState {
            position: x.to_string(),
            step: 0,
        })
        .collect_vec();

    let mut min_step = 0;
    let mut max_step = 0;

    let mut cache: HashMap<(usize, String), PositionState> = HashMap::new();

    // First step everything forward so that it is at a Z state
    for state in &mut states {
        let new_state = move_to_next_z(state, &instructions, &transitions, &mut cache);
        state.step = new_state.step;
        state.position = new_state.position;
        max_step = std::cmp::max(state.step, max_step);
    }

    loop {
        if min_step == max_step {
            break;
        }

        // Noticed that you get into a state where a state moves to the same state, so should just use the LCM
        // though never got a chance to implement this since Rust found the solution through brute force first

        for state in &mut states {
            while state.step < max_step {
                let new_state = move_to_next_z(state, &instructions, &transitions, &mut cache);

                state.step = new_state.step;
                state.position = new_state.position;
                max_step = std::cmp::max(state.step, max_step);
            }
        }

        min_step = states.iter().map(|state| state.step).min().unwrap();
    }

    Ok(max_step as u64)
}

fn move_to_next_z(
    state: &PositionState,
    instructions: &[char],
    transitions: &HashMap<String, HashMap<char, String>>,
    cache: &mut HashMap<(usize, String), PositionState>,
) -> PositionState {
    let mod_step_start = state.step % instructions.len();
    if let Some(cached_state) = cache.get(&(mod_step_start, state.position.to_string())) {
        return PositionState {
            step: cached_state.step + state.step,
            position: cached_state.position.to_string(),
        };
    }

    let mut step = state.step;
    let mut position = &state.position;
    loop {
        let direction = instructions[step % instructions.len()];
        step += 1;
        position = &transitions[position][&direction];

        if position.ends_with('Z') {
            break;
        }
    }

    let new_state = PositionState {
        position: position.to_string(),
        step,
    };

    cache.insert(
        (mod_step_start, state.position.to_string()),
        PositionState {
            position: new_state.position.to_string(),
            step: step - state.step,
        },
    );
    new_state
}

fn tests() -> anyhow::Result<()> {
    let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";

    let solution = solve(input)?;

    assert_eq!(solution, 6);

    Ok(())
}
