use anyhow::Result;
use itertools::Itertools;
use std::collections::HashMap;
use SpringState::*;

#[derive(Debug, Clone)]
enum SpringState {
    Operational,
    Broken,
    Unknown,
}

#[derive(Debug, Clone)]
struct Input {
    springs: Vec<char>,
    groups: Vec<usize>,
}

#[aoc::main]
fn solve(input: &str) -> Result<usize> {
    let lines = aoc::parse_list::<String>(input)?;
    let mut states: Vec<Input> = Vec::new();
    for line in lines {
        let l = line.split_whitespace().collect_vec();
        dbg!(l[0]);
        dbg!(l[1]);

        let groups = l[1]
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect_vec();

        states.push(Input {
            springs: l[0].chars().collect_vec(),
            groups,
        });
    }

    println!("{:?}", states);

    let mut answer = 0;
    for state in states {
        answer += count_valid_combinations(&state);
    }

    // println!("{:?}", positions);

    Ok(answer)
}

fn count_valid_combinations(input: &Input) -> usize {
    let mut combinations = 0;
    if let Some(pos) = input.springs.iter().position(|c| *c == '?') {
        for c in ['.', '#'] {
            let mut input = input.clone();
            input.springs[pos] = c;
            if is_possible_combination(&input) {
                combinations += count_valid_combinations(&input);
            }
        }
    } else {
        if is_valid_combination(&input) {
            combinations += 1;
        }
    }

    // dbg!(combinations);
    combinations
}

fn is_possible_combination(input: &Input) -> bool {
    let mut g = input.groups.clone();

    let s = input.springs.iter().collect::<String>();
    let splits = s.split('.').filter(|x| !x.is_empty()).collect_vec();

    let mut gi = 0;
    let gl = g.len();
    for split in &splits {
        if split.contains('?') {
            // println!("May be valid: {:?}", input);
            return true;
        }
        if gi >= gl || split.len() != g[gi] {
            // println!("Invalid: {:?}", input);
            return false;
        }
        gi += 1;
    }
    if gi < gl {
        return false;
    }

    // println!("Valid: {:?}", input);
    true
}

fn is_valid_combination(input: &Input) -> bool {
    let mut g = input.groups.clone();

    let s = input.springs.iter().collect::<String>();
    let splits = s.split('.').filter(|x| !x.is_empty()).collect_vec();

    let mut gi = 0;
    let gl = g.len();
    for split in &splits {
        if split.contains('?') {
            return false;
        }
        if gi >= gl || split.len() != g[gi] {
            return false;
        }
        gi += 1;
    }
    if gi < gl {
        return false;
    }

    true
}

fn tests() -> anyhow::Result<()> {
    let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";

    let solution = solve(input)?;

    assert_eq!(solution, 21);

    Ok(())
}
