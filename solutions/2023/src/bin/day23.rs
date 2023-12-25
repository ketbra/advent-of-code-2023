use anyhow::Result;
use itertools::Itertools;
use lazy_regex::regex_captures;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use Direction::*;

#[derive(Debug, Clone, PartialEq, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, PartialEq)]
struct Hike {
    path: Vec<Pos>,
    seen: HashSet<Pos>,
}

impl Hike {
    fn neighbors(&self, map: &[Vec<char>]) -> Vec<Pos> {
        let mut neighbors = Vec::new();
        let last_pos = self.path.iter().last().unwrap();
        let height = map.len();
        let width = map[0].len();

        let possible_neighbors = match map[last_pos.row][last_pos.col] {
            '^' => vec![[0, -1]],
            'v' => vec![[0, 1]],
            '<' => vec![[-1, 0]],
            '>' => vec![[1, 0]],
            _ => vec![[0, 1], [0, -1], [1, 0], [-1, 0]],
        };

        let (i, j) = (last_pos.col as isize, last_pos.row as isize);
        for [di, dj] in possible_neighbors {
            let (new_row, new_col) = (last_pos.row as isize + dj, last_pos.col as isize + di);
            if new_row >= 0 && new_col >= 0 {
                let new_row = new_row as usize;
                let new_col = new_col as usize;
                let new_pos = Pos {
                    col: new_col,
                    row: new_row,
                };
                if new_row < height
                    && new_col < width
                    && !self.seen.contains(&new_pos)
                    && map[new_row][new_col] != '#'
                {
                    neighbors.push(new_pos);
                }
            }
        }

        neighbors
    }
}

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

    let height = map.len();
    let width = map[0].len();

    let valid_hikes = find_all_hikes(
        &map,
        Pos { row: 0, col: 1 },
        Pos {
            row: height - 1,
            col: width - 2,
        },
    );

    for hike in &valid_hikes {
        println!("Len={}", hike.path.len() - 1);
        // print_hike(hike, &map);

        // println!();
    }

    let answer = valid_hikes
        .iter()
        .map(|hike| hike.path.len() - 1)
        .max()
        .unwrap();
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

fn print_hike(hike: &Hike, map: &[Vec<char>]) {
    for (j, row) in map.iter().enumerate() {
        for (i, c) in row.iter().enumerate() {
            if hike.seen.contains(&Pos { row: j, col: i }) && *c == '.' {
                print!("O");
            } else {
                print!("{}", c);
            }
        }
        println!();
    }
}

fn find_all_hikes(map: &[Vec<char>], start_pos: Pos, end_pos: Pos) -> Vec<Hike> {
    // Create a queue of hikes to consider and add one at the starting
    // position
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    seen.insert(start_pos.clone());
    queue.push_back(Hike {
        seen,
        path: vec![start_pos],
    });

    let mut valid_paths = Vec::new();

    while let Some(hike) = queue.pop_front() {
        let last_step = hike.path.iter().last().unwrap();
        if *last_step == end_pos {
            valid_paths.push(hike);
        } else {
            for neighbor in hike.neighbors(map) {
                let mut hike = hike.clone();
                hike.seen.insert(neighbor.clone());
                hike.path.push(neighbor);
                queue.push_back(hike);
            }
        }
    }

    valid_paths
}

fn tests() -> anyhow::Result<()> {
    let input = r"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
";

    let solution = solve(input)?;

    assert_eq!(solution, 94);

    Ok(())
}
