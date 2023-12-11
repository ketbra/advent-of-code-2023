use anyhow::Result;
use itertools::Itertools;
use std::collections::HashMap;
use Direction::*;

#[derive(Debug, Clone, PartialEq, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[aoc::main]
fn solve(input: &str) -> Result<usize> {
    let map = aoc::parse_list::<String>(input)?
        .iter()
        .map(|x| x.chars().collect_vec())
        .to_owned()
        .collect_vec();

    print_map(&map);

    println!();

    let map = expand_map(&map);

    print_map(&map);

    // Get position of each galaxy
    let positions = get_galaxy_positions(&map);

    println!("Found {} galaxies", positions.len());

    let mut answer = 0;
    for galaxy in &positions {
        for other in &positions {
            // Only count the combination once
            if galaxy.0 > other.0 || (galaxy.0 == other.0 && galaxy.1 > other.1) {
                println!(
                    "Distance from {:?} to {:?} is {}",
                    galaxy,
                    other,
                    galaxy.0.abs_diff(other.0) + galaxy.1.abs_diff(other.1)
                );
                answer += galaxy.0.abs_diff(other.0) + galaxy.1.abs_diff(other.1);
            }
        }
    }

    println!("{:?}", positions);

    Ok(answer)
}

fn get_galaxy_positions(map: &[Vec<char>]) -> Vec<(usize, usize)> {
    let mut points = Vec::new();
    for (j, row) in map.iter().enumerate() {
        for (i, c) in row.iter().enumerate() {
            if *c == '#' {
                points.push((i, j));
            }
        }
    }

    points
}

fn expand_map(map: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut new_map = Vec::new();

    // Expand rows
    for row in map {
        new_map.push(row.clone());
        if row.iter().all(|x| *x == '.') {
            new_map.push(row.clone());
        }
    }

    // Expand columns
    for i in (0..(map[0].len() - 1)).rev() {
        if map.iter().all(|row| row[i] == '.') {
            // Duplicate column
            for row in new_map.iter_mut() {
                row.insert(i, '.');
            }
        }
    }

    new_map
}

fn print_map(map: &[Vec<char>]) {
    for row in map {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}

fn tests() -> anyhow::Result<()> {
    let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";

    let solution = solve(input)?;

    assert_eq!(solution, 374);

    Ok(())
}
