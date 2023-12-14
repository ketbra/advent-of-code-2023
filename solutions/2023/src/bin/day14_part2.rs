use anyhow::Result;
use itertools::Itertools;

#[aoc::main]
fn solve(input: &str) -> Result<usize> {
    let mut map = aoc::parse_list::<String>(input)?
        .iter()
        .map(|x| x.chars().collect_vec())
        .to_owned()
        .collect_vec();

    // Get to steady state
    let mut loads = Vec::new();
    let primer = 1000;
    for _ in 0..primer {
        map = cycle(&map);
        // dbg!(calculate_load(&map));
        loads.push(calculate_load(&map));
    }

    // Calculate cycle length
    let mut interval = 999999999;
    'INTERVAL: for i in 1..1000 {
        let depth = 10;
        for d in 0..depth {
            if loads[loads.len() - 1 - d - i] != loads[loads.len() - 1 - d] {
                continue 'INTERVAL;
            }
            interval = i;
            break 'INTERVAL;
        }
    }

    // dbg!(interval);

    // Jump ahead that many intervals
    let remainder = (1_000_000_000 - primer) % interval;

    // This seems unnecessary given the input data, but this makes the
    // solution more robust
    for _ in 0..remainder {
        map = cycle(&map);
    }

    let answer = calculate_load(&map);

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

fn tilt_map_north(map: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut new_map = map.to_vec();
    let width = map[0].len();
    let height = map.len();

    // Move each rock
    for j in 1..height {
        for i in 0..width {
            if new_map[j][i] == 'O' {
                let mut pos = j;
                for h in (0..j).rev() {
                    if new_map[h][i] == '.' {
                        pos = h;
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

fn tilt_map_south(map: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut new_map = map.to_vec();
    let width = map[0].len();
    let height = map.len();

    // Move each rock
    for j in (0..(height - 1)).rev() {
        for i in 0..width {
            if new_map[j][i] == 'O' {
                let mut pos = j;
                for h in (j + 1)..height {
                    if new_map[h][i] == '.' {
                        pos = h;
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

fn cycle(map: &[Vec<char>]) -> Vec<Vec<char>> {
    let map = tilt_map_north(&map);
    let map = tilt_map_west(&map);
    let map = tilt_map_south(&map);
    tilt_map_east(&map)
}

fn tilt_map_west(map: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut new_map = map.to_vec();
    let width = map[0].len();
    let height = map.len();

    // Move each rock
    for i in 1..width {
        for j in 0..height {
            if new_map[j][i] == 'O' {
                // println!("{i}, {j} is a stone");
                let mut pos = i;
                for w in (0..i).rev() {
                    // println!("Checking {}, {}", w, j);
                    if new_map[j][w] == '.' {
                        pos = w;
                        // println!("Sliding to {}, {}", pos, j);
                    } else {
                        break;
                    }
                }
                if pos != i {
                    new_map[j][pos] = 'O';
                    new_map[j][i] = '.';
                }
            }
        }
    }

    new_map
}

fn tilt_map_east(map: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut new_map = map.to_vec();
    let width = map[0].len();
    let height = map.len();

    // Move each rock
    for i in (0..(width - 1)).rev() {
        for j in 0..height {
            if new_map[j][i] == 'O' {
                let mut pos = i;
                for w in (i + 1)..width {
                    if new_map[j][w] == '.' {
                        pos = w;
                    } else {
                        break;
                    }
                }
                if pos != i {
                    new_map[j][pos] = 'O';
                    new_map[j][i] = '.';
                }
            }
        }
    }

    new_map
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

    assert_eq!(solution, 64);

    Ok(())
}
