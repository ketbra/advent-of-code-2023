use anyhow::Result;
use itertools::Itertools;
use std::collections::HashMap;

#[aoc::main]
fn solve(input: &str) -> Result<i64> {
    let sequences = aoc::parse_list::<String>(input)?
        .iter()
        .map(|x| {
            x.split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let mut answer = 0;
    for sequence in sequences {
        let mut first_nums = Vec::new();
        let mut sequence = sequence;
        while !sequence.iter().all(|x| *x == 0) {
            let next_sequence = get_next_sequence(&sequence);
            first_nums.push(*sequence.first().unwrap());
            sequence = next_sequence;
        }

        let mut next_num = 0;
        for x in first_nums.iter().rev() {
            next_num = *x - next_num;
        }

        answer += next_num;
    }

    Ok(answer)
}

fn get_next_sequence(sequence: &[i64]) -> Vec<i64> {
    sequence.windows(2).map(|w| w[1] - w[0]).collect_vec()
}

fn tests() -> anyhow::Result<()> {
    let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

    let solution = solve(input)?;

    assert_eq!(solution, 2);

    Ok(())
}
