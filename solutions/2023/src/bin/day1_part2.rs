use anyhow::Result;
use std::collections::HashMap;

#[aoc::main]
fn solve(input: &str) -> Result<u32> {
    let vals: Vec<String> = aoc::parse_list(input)?;

    let num_map = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    let mut answer = 0;
    for val in &vals {
        let (_, d1) = lazy_regex::regex_captures!(
            r#"(\d|one|two|three|four|five|six|seven|eight|nine)"#,
            val
        )
        .unwrap();
        let (_, d2) = lazy_regex::regex_captures!(
            r#".*(\d|one|two|three|four|five|six|seven|eight|nine)"#,
            val
        )
        .unwrap();
        let d1 = if d1.len() > 1 { num_map[d1] } else { d1 };
        let d2 = if d2.len() > 1 { num_map[d2] } else { d2 };
        let num = format!("{}{}", d1, d2);
        answer += num.parse::<u32>()?;
    }
    Ok(answer)
}

fn tests() -> anyhow::Result<()> {
    let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";

    let solution = solve(input)?;

    assert_eq!(solution, 281);
    Ok(())
}
