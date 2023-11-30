use anyhow::Result;

#[aoc::main]
fn main(input: &str) -> Result<u32> {
    println!("Running day 1 with input of size {} bytes", input.len());

    let vals: Vec<u32> = aoc::parse_list(input)?;
    for num1 in &vals {
        for num2 in &vals {
            if num1 + num2 == 2020 {
                return Ok(num1 * num2);
            }
        }
    }

    Ok(5)
}

#[test]
fn test1() -> anyhow::Result<()> {
    let input = "1721
979
366
299
675
1456";

    let solution = solve(input)?;

    assert_eq!(solution, 514579);
    Ok(())
}
