use anyhow::Result;
use itertools::Itertools;
use lazy_regex::regex_captures;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Pos {
    row: usize,
    col: usize,
}

#[aoc::main]
fn solve(input: &str) -> Result<usize> {
    let map = aoc::parse_list::<String>(input)?
        .iter()
        .map(|x| x.chars().collect_vec())
        .to_owned()
        .collect_vec();

    let marked_map = mark_reachable(&map, 9000);
    print_map(&marked_map);

    let answer = count_marked_plots(&marked_map);

    Ok(answer)
}

fn print_map(map: &[Vec<char>]) {
    for row in map {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}

fn mark_reachable(map: &[Vec<char>], max_steps: usize) -> Vec<Vec<char>> {
    let mut new_map = map.to_vec();

    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();

    let height = map.len();
    let width = map[0].len();

    let mut pos = None;
    // Find the starting position
    for j in 0..height {
        for i in 0..width {
            if map[j][i] == 'S' {
                pos = Some(Pos { row: j, col: i });
                new_map[j][i] = 'O';
                break;
            }
        }
    }

    let pos = pos.unwrap();
    seen.insert((0, pos.clone()));
    queue.push_back((0, pos));

    while let Some((steps, pos)) = queue.pop_front() {
        if steps % 1000 == 0 {
            println!("{steps}");
        }
        for [dj, di] in [[0, 1], [0, -1], [1, 0], [-1, 0]] {
            let (new_row, new_col) = (pos.row as isize + dj, pos.col as isize + di);
            if new_row >= 0 && new_col >= 0 {
                let new_row = new_row as usize;
                let new_col = new_col as usize;
                if new_row < height && new_col < width {
                    let new_pos = Pos {
                        col: new_col,
                        row: new_row,
                    };

                    let new_steps = steps + 1;
                    let new_state = (new_steps, new_pos.clone());
                    if !seen.contains(&new_state) {
                        seen.insert(new_state);
                        if map[new_row][new_col] != '#' {
                            if new_steps == max_steps {
                                new_map[new_row][new_col] = 'O';
                            } else {
                                queue.push_back((new_steps, new_pos));
                                // new_map[new_row][new_col] = 'O';
                            }
                        }
                    }
                }
            }
        }
    }

    new_map
}

fn count_marked_plots(map: &[Vec<char>]) -> usize {
    map.iter().fold(0, |accum, row| {
        accum + row.iter().filter(|c| **c == 'O').count()
    })
}

fn tests() -> anyhow::Result<()> {
    let input = r"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
";

    let solution = solve(input)?;

    assert_eq!(solution, 42);

    Ok(())
}
