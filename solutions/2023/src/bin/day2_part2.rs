use anyhow::Result;
use std::cmp::max;

#[aoc::main]
fn solve(input: &str) -> Result<u32> {
    let vals: Vec<String> = aoc::parse_list(input)?;
    let mut answer = 0;
    for line in &vals {
        let (_, _, selections) = lazy_regex::regex_captures!(r#"^Game (\d+):(.+)"#, line).unwrap();
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        for selection in selections.split(';') {
            if selection.contains("red") {
                let (_, red) = lazy_regex::regex_captures!(r#"(\d+) red"#, selection).unwrap();
                let red = red.parse::<u32>()?;
                min_red = max(red, min_red);
            }
            if selection.contains("green") {
                let (_, green) = lazy_regex::regex_captures!(r#"(\d+) green"#, selection).unwrap();
                let green = green.parse::<u32>()?;
                min_green = max(green, min_green);
            }
            if selection.contains("blue") {
                let (_, blue) = lazy_regex::regex_captures!(r#"(\d+) blue"#, selection).unwrap();
                let blue = blue.parse::<u32>()?;
                min_blue = max(blue, min_blue);
            }
        }
        let power = min_red * min_green * min_blue;
        answer += power;
    }
    Ok(answer)
}

fn tests() -> anyhow::Result<()> {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

    let solution = solve(input)?;

    assert_eq!(solution, 2286);
    Ok(())
}
