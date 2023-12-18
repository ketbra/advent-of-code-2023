use anyhow::Result;
use itertools::Itertools;
use pathfinding::prelude::astar;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use Direction::*;

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Pos {
    row: isize,
    col: isize,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Cell {
    color: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Instruction {
    direction: Direction,
    distance: usize,
    color: String,
}

#[aoc::main]
fn solve(input: &str) -> Result<usize> {
    let instructions = aoc::parse_list::<String>(input)?
        .iter()
        .map(|line| {
            let v = line.split_whitespace().collect_vec();
            Instruction {
                direction: match v[0] {
                    "U" => North,
                    "L" => West,
                    "R" => East,
                    "D" => South,
                    x => panic!("Unknown direction, {x}"),
                },
                distance: v[1].parse::<usize>().unwrap(),
                color: v[2][1..(v[2].len() - 1)].to_string(),
            }
        })
        .collect_vec();

    // Create a sparse map and place us at 0,0
    let mut map: HashMap<Pos, Cell> = HashMap::new();
    let mut pos = Pos { row: 0, col: 0 };
    map.insert(
        pos.clone(),
        Cell {
            color: "#000000".to_string(),
        },
    );

    for instruction in &instructions {
        for i in 1..=instruction.distance {
            let mut new_pos = pos.clone();
            match instruction.direction {
                North => new_pos.row -= 1,
                South => new_pos.row += 1,
                East => new_pos.col += 1,
                West => new_pos.col -= 1,
            }

            pos = new_pos.clone();
            map.insert(
                new_pos,
                Cell {
                    color: instruction.color.to_string(),
                },
            );
        }
    }

    let answer = get_contained_size(&mut map);

    // println!("{map:?}");
    print_sparse_map(&map);

    Ok(answer)
}

fn get_dimensitons(map: &HashMap<Pos, Cell>) -> (Pos, Pos) {
    let positions = map.keys().collect_vec();

    //. Figure out the dimensions
    let mut min_row = positions[0].row;
    let mut min_col = positions[0].col;
    let mut max_row = positions[0].row;
    let mut max_col = positions[0].col;

    for pos in positions {
        min_row = pos.row.min(min_row);
        min_col = pos.col.min(min_col);
        max_row = pos.row.max(max_row);
        max_col = pos.col.max(max_col);
    }

    (
        Pos {
            row: min_row,
            col: min_col,
        },
        Pos {
            row: max_row,
            col: max_col,
        },
    )
}

fn get_contained_size(map: &mut HashMap<Pos, Cell>) -> usize {
    let (min, max) = get_dimensitons(map);
    let width = max.col - min.col + 1;
    let height = max.row - min.row + 1;

    // let mut seen = HashMap::new();

    let mut queue = VecDeque::new();
    let mut exterior = HashSet::new();

    // Paint from exterior
    for row in min.row..=max.row {
        for col in min.col..=max.col {
            if col == min.col || col == max.col || row == min.row || row == max.row {
                let pos = Pos { row, col };
                if !map.contains_key(&pos) {
                    queue.push_back(pos.clone());
                    exterior.insert(pos);
                }
            }
        }
    }

    while let Some(pos) = queue.pop_front() {
        for delta in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
            let new_pos = Pos {
                col: pos.col + delta.0,
                row: pos.row + delta.1,
            };
            let on_map = new_pos.col >= min.col
                && new_pos.col <= max.col
                && new_pos.row >= min.row
                && new_pos.row <= max.row;
            if on_map && !map.contains_key(&new_pos) && !exterior.contains(&new_pos) {
                exterior.insert(new_pos.clone());
                queue.push_back(new_pos);
            }
        }
    }

    let mut contained_size = 0;
    for row in min.row..=max.row {
        for col in min.col..=max.col {
            let pos = Pos { row, col };
            if !exterior.contains(&pos) {
                contained_size += 1;
            }
        }
    }

    contained_size
}

fn print_sparse_map(map: &HashMap<Pos, Cell>) {
    let (min, max) = get_dimensitons(map);

    // Shift the positions to make everything positive and convert to a dense map
    println!("{min:?} to {max:?}");

    let width = max.col - min.col + 1;
    let height = max.row - min.row + 1;

    for row in min.row..=max.row {
        for col in min.col..=max.col {
            if map.contains_key(&Pos { row, col }) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn tests() -> anyhow::Result<()> {
    let input = r"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
";

    let solution = solve(input)?;

    assert_eq!(solution, 62);

    Ok(())
}
