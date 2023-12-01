use anyhow::{bail, Result};

#[aoc::main]
fn solve(input: &str) -> Result<u32> {
    let vals: Vec<String> = aoc::parse_list(input)?;
    let mut answer = 0;

    println!("{:?}", vals);

    let mut answer = 0;
    for val in &vals {
        let text = lazy_regex::regex_replace_all!(r#"[^0-9]"#, &val, "").to_string();
        let chars: Vec<char> = text.chars().collect();
        let num = format!("{}{}", chars[0], chars[chars.len() - 1]);
        println!("{}", num);
        answer += num.parse::<u32>()?;
    }
    Ok(answer)
    // let (_, year, day, part) =
    //     lazy_regex::regex_captures!(r#"/(\d{4})/src/bin/day(\d+)(?:_part(\d))?.rs$"#, path)
    //         .context("File must be named YYYY/src/bin/day\\d+.rs")?;

    // bail!("No solution found")
}

fn tests() -> anyhow::Result<()> {
    let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    let solution = solve(input)?;

    assert_eq!(solution, 142);
    Ok(())
}
