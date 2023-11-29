use anyhow::Result;

#[aoc::main]
fn main(input: &str) -> Result<u32> {
    println!("Running day 1 with input of size {} bytes", input.len());
    Ok(1)
}

#[test]
fn test1() -> anyhow::Result<()> {
    let input = "";

    let solution = solve(input)?;

    assert_eq!(solution, 999999);
    Ok(())
}
