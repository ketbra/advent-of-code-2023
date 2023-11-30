use anyhow::{bail, Result};

#[aoc::main]
fn solve(input: &str) -> Result<u32> {
    let vals: Vec<u32> = aoc::parse_list(input)?;
    let mut answer = 0;
    for num1 in &vals {}

    bail!("No solution found")
}

fn tests() -> anyhow::Result<()> {
    let input = "";

    let solution = solve(input)?;

    assert_eq!(solution, 314159);
    Ok(())
}
