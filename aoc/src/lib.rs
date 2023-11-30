use anyhow::{Context, Result};
pub use aoc_macro::main;
use clap::Parser;
use std::fmt::Display;
use std::str::FromStr;
use std::vec::Vec;

/// Arguments
#[derive(Parser)]
pub struct Cli {
    #[arg(long)]
    pub submit: bool,
}

// Define our error types. These may be customized for our error handling cases.
// Now we will be able to write our own errors, defer to an underlying error
// implementation, or do something in between.
#[derive(Debug, Clone)]
pub struct NoSolutionError;

// Generation of an error is completely separate from how it is displayed.
// There's no need to be concerned about cluttering complex logic with the display style.
//
// Note that we don't store any extra info about the errors. This means we can't state
// which string failed to parse without modifying our types to carry that information.
impl std::fmt::Display for NoSolutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Failed to find a solution")
    }
}

pub fn get_cli_args() -> Cli {
    Cli::parse()
}

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

pub fn submit_script<T>(path: &str, answer: &T) -> Result<()>
where
    T: Display + ?Sized,
{
    let puzzle = puzzle_from_file_name(path)?;
    submit(puzzle.day, puzzle.year, &answer.to_string(), puzzle.part)
}

fn submit(day: usize, year: usize, answer: &str, part: u8) -> Result<()> {
    elv::submit(day, year, answer, part, None)
}

struct Puzzle {
    day: usize,
    year: usize,
    part: u8,
}

fn puzzle_from_file_name(path: &str) -> Result<Puzzle> {
    let (_, year, day, part) =
        lazy_regex::regex_captures!(r#"/(\d{4})/src/bin/day(\d+)(?:_part(\d))?.rs$"#, path)
            .context("File must be named YYYY/src/bin/day\\d+.rs")?;
    let day = day
        .parse::<usize>()
        .with_context(|| format!("Failed to parse day, {}", day))?;
    let year = year
        .parse::<usize>()
        .with_context(|| format!("Failed to parse year, {}", year))?;

    let part = if !part.is_empty() {
        part.parse::<u8>()
            .with_context(|| format!("Failed to parse part, {}", part))?
    } else {
        1
    };

    Ok(Puzzle { day, year, part })
}

pub fn get_input_for_script(path: &str) -> Result<String> {
    let puzzle = puzzle_from_file_name(path)?;

    let home_dir = std::env::var("HOME")?;
    let cache_fn = format!("{}/.aoc/{}/{}.inp", home_dir, puzzle.year, puzzle.day);
    println!("Cache file={}", cache_fn);
    let input_cache_path = std::path::Path::new(&cache_fn);
    if !input_cache_path.exists() {
        // Create parent folder
        println!("Creating {:?}", input_cache_path.parent().unwrap());
        std::fs::create_dir_all(input_cache_path.parent().unwrap()).unwrap();

        // Download and save the data
        let input = get_input(puzzle.day, puzzle.year)?;
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
