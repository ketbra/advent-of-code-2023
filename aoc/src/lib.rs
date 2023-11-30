use anyhow::{Context, Result};
pub use aoc_macro::main;
use std::str::FromStr;
use std::vec::Vec;

pub fn parse_u32_list(input: &str) -> Result<Vec<u32>> {
    let vals: Result<Vec<_>, _> = input.lines().map(|val| val.parse::<u32>()).collect();
    Ok(vals?)
}

pub fn parse_list<T>(input: &str) -> std::result::Result<Vec<T>, <T as FromStr>::Err>
where
    T: FromStr,
{
    let vals: Result<Vec<_>, _> = input.lines().map(|val| val.parse::<T>()).collect();
    vals
}

fn get_input(day: usize, year: usize) -> Result<String> {
    elv::get_input(day, year, None)
}

pub fn get_input_for_script(path: &str) -> Result<String> {
    let (_, year, day) = lazy_regex::regex_captures!(r#"/(\d{4})/src/bin/day(\d+).rs$"#, path)
        .context("File must be named YYYY/src/bin/day\\d+.rs")?;
    let day = day
        .parse::<usize>()
        .with_context(|| format!("Failed to parse day, {}", day))?;
    let year = year
        .parse::<usize>()
        .with_context(|| format!("Failed to parse year, {}", year))?;

    let home_dir = std::env::var("HOME")?;
    let cache_fn = format!("{}/.aoc/{}/{}.inp", home_dir, year, day);
    println!("Cache file={}", cache_fn);
    let input_cache_path = std::path::Path::new(&cache_fn);
    if !input_cache_path.exists() {
        // Create parent folder
        println!("Creating {:?}", input_cache_path.parent().unwrap());
        std::fs::create_dir_all(input_cache_path.parent().unwrap()).unwrap();

        // Download and save the data
        let input = get_input(day, year)?;
        std::fs::File::create(input_cache_path)?;
        std::fs::write(input_cache_path, input)?;
    }

    // Slurp the cache file
    let input = std::fs::read_to_string(input_cache_path)?;
    Ok(input)
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
