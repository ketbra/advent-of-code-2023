use anyhow::Result;
use itertools::Itertools;
use std::collections::HashMap;

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

    println!("{:?}", transitions);

    let mut answer = 0;
    let mut i = 0;
    let mut position = "AAA";
    loop {
        println!("Now at {}", position);
        if position == "ZZZ" {
            break;
        }

        let direction = instructions[i % instructions.len()];
        println!("Will go {} from {}", direction, position);
        i += 1;
        answer += 1;
        position = &transitions[position][&direction];
    }

    println!("{:?}", instructions);
    println!("{:?}", transitions);

    Ok(answer)
}

fn tests() -> anyhow::Result<()> {
    let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";

    let solution = solve(input)?;

    assert_eq!(solution, 2);

    let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";

    let solution = solve(input)?;

    assert_eq!(solution, 6);

    Ok(())
}
