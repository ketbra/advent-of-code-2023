use anyhow::Result;
use std::collections::HashMap;

fn str_rev(s: &str) -> String {
    s.chars().rev().collect::<String>()
}

#[aoc::main]
fn solve(input: &str) -> Result<u32> {
    let vals: Vec<String> = aoc::parse_list(input)?;

    // println!("{:?}", vals);

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
        let text = lazy_regex::regex_replace!(
            r#"^[^0-9]*?(one|two|three|four|five|six|seven|eight|nine)"#,
            val,
            |_, name| num_map[name]
        )
        .to_string();
        let text = lazy_regex::regex_replace!(
            r#"^[^0-9]*?(enin|thgie|neves|xis|evif|ruof|eerht|owt|eno)"#,
            str_rev(&text).as_str(),
            |_, name| num_map[str_rev(name).as_str()]
        )
        .to_string();
        let text = str_rev(&text);
        let text = lazy_regex::regex_replace_all!(r#"[^0-9]"#, &text, "").to_string();
        let chars: Vec<char> = text.chars().collect();
        let num = format!("{}{}", chars[0], chars[chars.len() - 1]);
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
