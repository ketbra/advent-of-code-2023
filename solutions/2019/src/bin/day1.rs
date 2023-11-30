use anyhow::{bail, Result};

#[aoc::main]
fn solve(input: &str) -> Result<i64> {
    let vals: Vec<i64> = aoc::parse_list(input)?;
    let mut answer = 0;
    for x in &vals {
        answer += (*x as f64 / 3.0).floor() as i64 - 2
    }
    Ok(answer)

    // bail!("No solution found")
}

fn tests() -> anyhow::Result<()> {
    let input = "12
14
1969
100756
";

    let solution = solve(input)?;

    assert_eq!(solution, 2 + 2 + 654 + 33583);
    Ok(())
}
