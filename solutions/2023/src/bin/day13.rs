use anyhow::Result;
use itertools::Itertools;
use std::collections::HashMap;

#[aoc::main]
fn solve(input: &str) -> Result<usize> {
    let lines = aoc::parse_list::<String>(input)?;

    let puzzles = lines
        .split(|s| s.is_empty())
        .map(|rows| {
            rows.iter()
                .map(|row| row.chars().collect_vec())
                .collect_vec()
        })
        .collect_vec();

    print_map(&puzzles[0]);

    let mut answer = 0;
    for puzzle in puzzles {
        if let Some(index) = find_vertical_reflextion(&puzzle) {
            println!("Vertical reflextion at index {index}");

            answer += index + 1;
        } else if let Some(index) = find_horizontal_reflextion(&puzzle) {
            println!("Horizontal reflextion at index {index}");
            answer += 100 * (index + 1);
        } else {
            print_map(&puzzle);
            panic!("Failed to find reflextion in above puzzle");
        }
    }

    // println!("{:?}", puzzles);

    // let mut lines: Vec<String> = Vec::new();

    // println!("{:?}", positions);

    Ok(answer)
}

fn find_vertical_reflextion(map: &[Vec<char>]) -> Option<usize> {
    let i = 0;

    let width = map[0].len();

    'COL: for col in 0..width - 1 {
        let mut col1 = col;
        let mut col2 = col + 1;
        loop {
            if !cols_equal(col1, col2, map) {
                continue 'COL;
            }

            if col1 < 1 || col2 >= width - 1 {
                break;
            }

            col1 -= 1;
            col2 += 1;
        }
        return Some(col);
    }
    None
}

fn find_horizontal_reflextion(map: &[Vec<char>]) -> Option<usize> {
    let i = 0;

    let height = map.len();

    'ROW: for row in 0..map.len() - 1 {
        let mut row1 = row;
        let mut row2 = row + 1;
        println!("Considering between {row1} and {row2}");
        loop {
            if !rows_equal(row1, row2, map) {
                println!("{row1} != {row2} so breaking");
                continue 'ROW;
            }

            if row1 < 1 || row2 >= height - 1 {
                break;
            }

            row1 -= 1;
            row2 += 1;
        }
        return Some(row);
    }
    None
}

fn rows_equal(row1: usize, row2: usize, map: &[Vec<char>]) -> bool {
    map[row1] == map[row2]
}

fn cols_equal(col1: usize, col2: usize, map: &[Vec<char>]) -> bool {
    for row in map {
        if row[col1] != row[col2] {
            return false;
        }
    }
    true
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
    let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";

    let solution = solve(input)?;

    assert_eq!(solution, 405);

    Ok(())
}
