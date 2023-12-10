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

    // Find starting position
    let mut start_pos = (0, 0);
    'START_SEARCH: for (j, row) in map.iter().enumerate() {
        for (i, cell) in map[j].iter().enumerate() {
            if *cell == 'S' {
                start_pos = (i, j);
                break 'START_SEARCH;
            }
        }
    }

    println!("Start at {:?}", start_pos);

    let (length, path_map) = [
        ('|', South),
        ('-', West),
        ('L', East),
        ('J', West),
        ('7', South),
        ('F', South),
    ]
    .iter()
    .filter_map(|c| get_path_size(&(start_pos.0, start_pos.1, c.1), &c.0, &map))
    .next()
    .unwrap();

    let path_length: usize = path_map
        .iter()
        .map(|row| row.iter().filter(|x| **x != '.').count())
        .sum();
    println!("{length} vs {path_length}");

    let path_map = mark_all_reachable_points(&path_map);

    let interior_points: usize = path_map
        .iter()
        .map(|row| row.iter().filter(|x| **x == '.').count())
        .sum();

    Ok(interior_points)
}

fn mark_all_reachable_points(path_map: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut map = Vec::new();

    // First enlarge the map so that we can handle connections between cells
    for row in path_map {
        let mut new_row = Vec::new();
        let mut spacer_row = Vec::new();
        for c in row {
            new_row.push(*c);
            new_row.push('*');
            spacer_row.push('*');
            spacer_row.push('*');
        }
        map.push(new_row);
        map.push(spacer_row);
    }

    // First mark the initial exterior points
    let row_count = map.len();
    let col_count = map[0].len();

    let map_clone = map.to_vec();
    for (j, row) in map.iter_mut().enumerate() {
        for (i, c) in row.iter_mut().enumerate() {
            if j == 0 || i == 0 || j == row_count - 1 || i == col_count - 1 {
                if *c == '.' || (*c == '*' && can_squeeze_through(i, j, &map_clone)) {
                    *c = 'O';
                }
            }
        }
    }

    loop {
        let mut updates = Vec::new();
        for (j, row) in map.iter().enumerate() {
            for (i, c) in row.iter().enumerate() {
                if *c == 'O' {
                    for (x, y) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                        let (x, y) = (i + x as usize, j + y as usize);
                        if let Some(c) = map.get(y).and_then(|row| row.get(x)) {
                            if *c == '.' || (*c == '*' && can_squeeze_through(x, y, &map)) {
                                updates.push((x, y));
                            }
                        }
                    }
                }
            }
        }

        if updates.is_empty() {
            break;
        }

        for update in updates {
            map[update.1][update.0] = 'O';
        }
    }

    // println!("{:?}", map);
    // print_map(path_map);
    // print_map(&map);

    map
}

fn get_neighbor(
    x: usize,
    y: usize,
    map: &[Vec<char>],
    direction: Direction,
) -> Option<(usize, usize, char)> {
    let diff: (isize, isize) = match direction {
        North => (0, -1),
        South => (0, 1),
        East => (1, 0),
        West => (-1, 0),
    };

    let (x, y) = (x + diff.0 as usize, y + diff.1 as usize);
    if let Some(c) = map.get(y).and_then(|row| row.get(x)) {
        return Some((x, y, *c));
    }
    None
}

fn can_squeeze_through(x: usize, y: usize, map: &[Vec<char>]) -> bool {
    // Make sure above and below don't connect
    let north = get_neighbor(x, y, map, North);
    let south = get_neighbor(x, y, map, South);
    let east = get_neighbor(x, y, map, East);
    let west = get_neighbor(x, y, map, West);

    if let (Some(west), Some(east)) = (west, east) {
        if matches!((west.2, east.2), ('-' | 'F' | 'L', '-' | 'J' | '7')) {
            return false;
        }
    }

    if let (Some(north), Some(south)) = (north, south) {
        if matches!((north.2, south.2), ('|' | 'F' | '7', '|' | 'J' | 'L')) {
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

fn get_path_size(
    start_pos: &(usize, usize, Direction),
    start_char: &char,
    map: &[Vec<char>],
) -> Option<(usize, Vec<Vec<char>>)> {
    let mut steps = 0;
    let mut pos = start_pos.to_owned();

    let mut map = map.to_vec();
    map[start_pos.1][start_pos.0] = *start_char;

    let mut path_map = map
        .iter()
        .map(|row| row.iter().map(|x| '.').collect_vec())
        .collect_vec();

    println!("Starting at {:?}, {start_char}", pos);
    while let Some(new_pos) = next_pos(&pos, &map) {
        pos = new_pos;
        steps += 1;

        path_map[pos.1][pos.0] = map[pos.1][pos.0];

        if pos.0 == start_pos.0 && pos.1 == start_pos.1 && steps > 0 {
            // Make sure the starting position can be entered from the direction taken
            if can_enter(map[pos.1][pos.0], pos.2) {
                return Some((steps, path_map));
            }
            return None;
        }
    }

    None
}

fn can_enter(c: char, d: Direction) -> bool {
    match (c, d) {
        ('|' | '7' | 'F', South) => true,
        ('|' | 'L' | 'J', North) => true,
        ('-' | 'J' | '7', West) => true,
        ('-' | 'L' | 'F', East) => true,
        _ => false,
    }
}

fn next_pos(
    start_pos: &(usize, usize, Direction),
    map: &[Vec<char>],
) -> Option<(usize, usize, Direction)> {
    let s_char = map[start_pos.1][start_pos.0];
    let diff: Option<(isize, isize, Direction)> = match (s_char, start_pos.2.to_owned()) {
        ('|', d) => Some(d),
        ('-', d) => Some(d),
        ('L', North) => Some(West),
        ('L', East) => Some(South),
        ('J', West) => Some(South),
        ('J', North) => Some(East),
        ('7', West) => Some(North),
        ('7', South) => Some(East),
        ('F', East) => Some(North),
        ('F', South) => Some(West),
        _ => None,
    }
    .map(|dir| match dir {
        North => (0, 1, dir),
        South => (0, -1, dir),
        East => (-1, 0, dir),
        West => (1, 0, dir),
    });

    diff.map(|next| {
        (
            start_pos.0 as isize + next.0,
            start_pos.1 as isize + next.1,
            next.2,
        )
    })
    .filter(|pos| {
        map.get(pos.1 as usize)
            .and_then(|row| row.get(pos.0 as usize))
            .is_some()
    })
    .map(|p| (p.0 as usize, p.1 as usize, p.2))
}

fn tests() -> anyhow::Result<()> {
    let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
";

    let solution = solve(input)?;

    assert_eq!(solution, 4);

    let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
";

    let solution = solve(input)?;

    assert_eq!(solution, 8);

    let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";

    let solution = solve(input)?;

    assert_eq!(solution, 10);
    Ok(())
}
