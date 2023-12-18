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
struct Cell {}

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
        .map(|line| parse_instruction(line))
        .collect_vec();

    // First follow the path and create a list of all of the row and column indexes
    let mut row_vals = HashSet::new();
    let mut col_vals = HashSet::new();
    let mut pos = Pos { row: 0, col: 0 };
    for instruction in &instructions {
        let mut new_pos = pos.clone();
        match instruction.direction {
            North => new_pos.row -= instruction.distance as isize,
            South => new_pos.row += instruction.distance as isize,
            East => new_pos.col += instruction.distance as isize,
            West => new_pos.col -= instruction.distance as isize,
        }
        pos = new_pos;
        row_vals.insert(pos.row);
        col_vals.insert(pos.col);
    }

    // For each value, add one less than the number.  This will allow us to separate lines later
    for x in row_vals.clone() {
        row_vals.insert(x - 1);
    }
    for x in col_vals.clone() {
        col_vals.insert(x - 1);
    }

    let mut row_vals = row_vals.iter().copied().sorted().collect_vec();
    let mut col_vals = col_vals.iter().copied().sorted().collect_vec();

    // Extend past the last values as well
    row_vals.push(row_vals.last().unwrap() + 1);
    col_vals.push(col_vals.last().unwrap() + 1);

    // Create a mapping of actual column to index
    let mut c2i: HashMap<isize, usize> = HashMap::new();
    for (i, c) in col_vals.iter().enumerate() {
        c2i.insert(*c, i);
    }

    // Create a mapping of actual row to index
    let mut r2i: HashMap<isize, usize> = HashMap::new();
    for (i, r) in row_vals.iter().enumerate() {
        r2i.insert(*r, i);
    }

    // Generate the compressed path
    let mut map: HashMap<Pos, Cell> = HashMap::new();
    // let mut pos = Pos { row: 0, col: 0 };
    // map.insert(pos.clone(), Cell {});
    for instruction in &instructions {
        let mut new_pos = pos.clone();
        match instruction.direction {
            North => new_pos.row -= instruction.distance as isize,
            South => new_pos.row += instruction.distance as isize,
            East => new_pos.col += instruction.distance as isize,
            West => new_pos.col -= instruction.distance as isize,
        }

        let c_1 = *c2i.get(&pos.col).unwrap() as isize;
        let r_1 = *r2i.get(&pos.row).unwrap() as isize;
        let c_2 = *c2i.get(&new_pos.col).unwrap() as isize;
        let r_2 = *r2i.get(&new_pos.row).unwrap() as isize;

        let c_min = c_1.min(c_2);
        let r_min = r_1.min(r_2);
        let c_max = c_1.max(c_2);
        let r_max = r_1.max(r_2);

        // println!("Moving from {c_min},{r_min} to {c_max}, {r_max}");

        for ccol in c_min..=c_max {
            for crow in r_min..=r_max {
                map.insert(
                    Pos {
                        row: crow,
                        col: ccol,
                    },
                    Cell {},
                );
            }
        }
        pos = new_pos;
    }

    let answer = get_contained_size(&mut map, &col_vals, &row_vals);

    // print_sparse_map(&map);

    Ok(answer)
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

fn parse_instruction(s: &str) -> Instruction {
    let v = s.split_whitespace().collect_vec();
    let color = v[2][2..(v[2].len() - 1)].to_string();
    let distance = color[..(color.len() - 1)].to_string();
    let distance = usize::from_str_radix(&distance, 16).unwrap();
    let direction = match color.chars().last().unwrap() {
        '0' => East,
        '1' => South,
        '2' => West,
        '3' => North,
        x => panic!("Unknown direction indicator, {x}"),
    };

    Instruction {
        distance,
        direction,
        color: "".to_string(),
    }
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

fn get_contained_size(
    map: &mut HashMap<Pos, Cell>,
    col_vals: &Vec<isize>,
    row_vals: &Vec<isize>,
) -> usize {
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
                let length = col_vals[col as usize] - col_vals[col as usize - 1];
                let width = row_vals[row as usize] - row_vals[row as usize - 1];

                contained_size += length * width;
            }
        }
    }

    contained_size as usize
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

    assert_eq!(solution, 952408144115);

    Ok(())
}
