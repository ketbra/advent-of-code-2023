use anyhow::Result;
use itertools::Itertools;
use std::collections::HashMap;

#[aoc::main]
fn solve(input: &str) -> Result<usize> {
    let map = aoc::parse_list::<String>(input)?
        .iter()
        .map(|x| x.chars().collect_vec())
        .to_owned()
        .collect_vec();

    print_map(&map);

    println!();

    let map = tilt_map(&map);
    print_map(&map);

    let mut answer = calculate_load(&map);

    Ok(answer)
}

fn calculate_load(map: &[Vec<char>]) -> usize {
    let width = map[0].len();
    let height = map.len();

    let mut load = 0;
    for j in 0..height {
        for i in 0..width {
            if map[j][i] == 'O' {
                load += height - j;
            }
        }
    }

    load
}

fn tilt_map(map: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut new_map = map.to_vec();
    let width = map[0].len();
    let height = map.len();

    // Move each rock
    for j in 1..height {
        for i in 0..width {
            if new_map[j][i] == 'O' {
                println!("{i}, {j} is a stone");
                let mut pos = j;
                for h in (0..j).rev() {
                    println!("Checking {}, {}", h, i);
                    if new_map[h][i] == '.' {
                        pos = h;
                        println!("Sliding to {}, {}", pos, i);
                    } else {
                        break;
                    }
                }
                if pos != j {
                    new_map[pos][i] = 'O';
                    new_map[j][i] = '.';
                }
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
    let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";

    let solution = solve(input)?;

    assert_eq!(solution, 136);

    Ok(())
}
