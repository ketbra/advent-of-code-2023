use anyhow::Result;
use cached::proc_macro::cached;
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
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

        let springs_orig = l[0].chars().collect_vec();
        let mut springs = springs_orig.clone();
        for i in 0..4 {
            springs.push('?');
            springs.extend(springs_orig.clone());
        }

        let groups = groups
            .iter()
            .cycle()
            .take(groups.len() * 5)
            .map(|c| *c)
            .collect_vec();

        states.push(Input { springs, groups });
    }

    println!("{:?}", states);
    // Expand state

    let mut answer = 0;
    let count = states.len();
    for (i, state) in states.iter().enumerate() {
        println!("Processing {i} of {count}");
        answer += count_valid_combinations(state.clone());
    }

    // println!("{:?}", positions);

    Ok(answer)
}

#[cached]
fn count_valid_combinations(input: Input) -> usize {
    let mut combinations = 0;
    if let Some(pos) = input.springs.iter().position(|c| *c == '?') {
        for c in ['.', '#'] {
            let mut input = input.clone();
            input.springs[pos] = c;

            // Shrink the input to only the parts that can vary to allow caching
            if is_possible_combination(&input) {
                let input = shrink_input(&input);
                combinations += count_valid_combinations(input);
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

fn shrink_input(input: &Input) -> Input {
    let s = input.springs.iter().collect::<String>();
    let splits = s.split('.').filter(|x| !x.is_empty()).collect_vec();

    for split in &splits {}

    let mut si = 0;
    'SPLITS: for (split_idx, split) in splits.iter().enumerate() {
        for (i, c) in split.chars().enumerate() {
            if c == '?' {
                si = split_idx;
                break 'SPLITS;
            }
        }
    }

    if si == 0 {
        return input.clone();
    }

    let springs = splits.iter().skip(si).join(".").chars().collect_vec();
    let groups = input.groups.iter().skip(si).map(|c| *c).collect_vec();

    let out = Input { springs, groups };
    // println!("Shrunk {:?} to {:?}", input, out);
    out
}

fn is_possible_combination(input: &Input) -> bool {
    let mut g = input.groups.clone();

    let s = input.springs.iter().collect::<String>();
    let splits = s.split('.').filter(|x| !x.is_empty()).collect_vec();

    let mut gi = 0;
    let gl = g.len();
    for split in &splits {
        for (i, c) in split.chars().enumerate() {
            if c == '?' {
                return true;
            }
            if gi >= gl || i >= g[gi] {
                return false;
            }
        }

        if split.len() != g[gi] {
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
    let g = input.groups.clone();
    // println!("Checking if {:?} is valid", input);
    let s = input.springs.iter().collect::<String>();
    let splits = s.split('.').filter(|x| !x.is_empty()).collect_vec();

    let mut gi = 0;
    let gl = g.len();
    for split in &splits {
        // Check leading hash size
        for (i, c) in split.chars().enumerate() {
            if c == '?' {
                return false;
            }
            if gi >= gl || i >= g[gi] {
                return false;
            }
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

    assert_eq!(solution, 525152);

    Ok(())
}
