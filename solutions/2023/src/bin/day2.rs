use anyhow::Result;

#[aoc::main]
fn solve(input: &str) -> Result<u32> {
    let vals: Vec<String> = aoc::parse_list(input)?;

    println!("{:?}", vals);

    let mut answer = 0;
    'LINE: for line in &vals {
        let (_, id, selections) = lazy_regex::regex_captures!(r#"^Game (\d+):(.+)"#, line).unwrap();
        let id = id.parse::<u32>()?;
        println!("{}", line);
        for selection in selections.split(';') {
            if selection.contains("red") {
                let (_, red) = lazy_regex::regex_captures!(r#"(\d+) red"#, selection).unwrap();
                let red = red.parse::<u32>()?;
                if red > 12 {
                    continue 'LINE; // invalid
                }
            }
            if selection.contains("green") {
                let (_, green) = lazy_regex::regex_captures!(r#"(\d+) green"#, selection).unwrap();
                let green = green.parse::<u32>()?;
                if green > 13 {
                    continue 'LINE; // invalid
                }
            }
            if selection.contains("blue") {
                let (_, blue) = lazy_regex::regex_captures!(r#"(\d+) blue"#, selection).unwrap();
                let blue = blue.parse::<u32>()?;
                if blue > 14 {
                    continue 'LINE; // invalid
                }
            }
        }
        println!("{} is valid", id);
        answer += id;
    }
    Ok(answer)
    // // let (_, year, day, part) =
    //     lazy_regex::regex_captures!(r#"/(\d{4})/src/bin/day(\d+)(?:_part(\d))?.rs$"#, path)
    //         .context("File must be named YYYY/src/bin/day\\d+.rs")?;

    // bail!("No solution found")
}

fn tests() -> anyhow::Result<()> {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

    let solution = solve(input)?;

    assert_eq!(solution, 8);
    Ok(())
}
