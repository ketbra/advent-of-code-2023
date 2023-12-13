use anyhow::Result;
use itertools::Itertools;
use std::collections::HashMap;
use Orientation::*;

#[derive(Debug, Clone, PartialEq)]
enum Orientation {
    Horizontal,
    Vertical,
}

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
    'PUZZLE: for puzzle in puzzles {
        let mut orig_reflextion = (0, Vertical);

        if let Some(index) = find_vertical_reflextion(&puzzle, None) {
            orig_reflextion = (index, Vertical);
            println!("Orig vertical reflextion at index {index}");
        } else if let Some(index) = find_horizontal_reflextion(&puzzle, None) {
            orig_reflextion = (index, Horizontal);
            println!("Orig horizontal reflextion at index {index}");
        } else {
            print_map(&puzzle);
            panic!("Failed to find reflextion in above puzzle");
        }

        let height = puzzle.len();
        let width = puzzle[0].len();

        for j in 0..height {
            for i in 0..width {
                let mut new_puzzle = puzzle.clone();

                new_puzzle[j][i] = match new_puzzle[j][i] {
                    '#' => '.',
                    '.' => '#',
                    _ => panic!("Unexpected puzzle character"),
                };

                let skip_index = if orig_reflextion.1 == Vertical {
                    Some(orig_reflextion.0)
                } else {
                    None
                };

                if let Some(index) = find_vertical_reflextion(&new_puzzle, skip_index) {
                    println!("New vertical reflextion at index {index}");

                    answer += index + 1;
                    continue 'PUZZLE;
                }

                let skip_index = if orig_reflextion.1 == Horizontal {
                    Some(orig_reflextion.0)
                } else {
                    None
                };
                if let Some(index) = find_horizontal_reflextion(&new_puzzle, skip_index) {
                    println!("New horizontal reflextion at index {index}");
                    answer += 100 * (index + 1);
                    continue 'PUZZLE;
                }
            }
        }
    }

    // println!("{:?}", puzzles);

    // let mut lines: Vec<String> = Vec::new();

    // println!("{:?}", positions);

    Ok(answer)
}

fn find_vertical_reflextion(map: &[Vec<char>], skip_index: Option<usize>) -> Option<usize> {
    let i = 0;

    let width = map[0].len();

    'COL: for col in 0..width - 1 {
        if let Some(x) = skip_index {
            if x == col {
                continue 'COL;
            }
        }

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

fn find_horizontal_reflextion(map: &[Vec<char>], skip_index: Option<usize>) -> Option<usize> {
    let i = 0;

    let height = map.len();

    'ROW: for row in 0..map.len() - 1 {
        if let Some(x) = skip_index {
            if x == row {
                continue 'ROW;
            }
        }

        let mut row1 = row;
        let mut row2 = row + 1;
        // println!("Considering between {row1} and {row2}");
        loop {
            if !rows_equal(row1, row2, map) {
                // println!("{row1} != {row2} so breaking");
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

    assert_eq!(solution, 400);

    Ok(())
}
