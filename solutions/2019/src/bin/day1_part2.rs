use anyhow::{bail, Result};

#[aoc::main]
fn solve(input: &str) -> Result<i64> {
    let vals: Vec<i64> = aoc::parse_list(input)?;
    let mut answer = 0;
    for x in &vals {
        let mut total = (*x as f64 / 3.0).floor() as i64 - 2;
        let mut last_fuel = total;
        while last_fuel > 0 {
            let mut fuel = (last_fuel as f64 / 3.0).floor() as i64 - 2;
            if fuel < 0 {
                fuel = 0;
            }
            total += fuel;
            last_fuel = fuel;
        }
        answer += total;
    }
    Ok(answer)

    // bail!("No solution found")
}

fn tests() -> anyhow::Result<()> {
    let input = "14
1969
100756
";

    let solution = solve(input)?;

    assert_eq!(solution, 2 + 966 + 50346);
    Ok(())
}
