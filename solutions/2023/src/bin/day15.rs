use anyhow::Result;
use itertools::Itertools;

#[aoc::main]
fn solve(input: &str) -> Result<usize> {
    let input = input.replace('\n', "");
    let inputs = input.split(',').collect_vec();

    let mut answer = 0;
    for input in inputs {
        answer += calculate_hash(input);
    }

    Ok(answer)
}

fn calculate_hash(s: &str) -> usize {
    let mut hash = 0;

    for c in s.chars() {
        hash += c as usize;
        hash *= 17;
        hash %= 256;
    }
    hash
}

fn tests() -> anyhow::Result<()> {
    let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
";

    let solution = solve(input)?;

    assert_eq!(solution, 1320);

    Ok(())
}
