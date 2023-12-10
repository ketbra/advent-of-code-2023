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
fn solve(input: &str) -> Result<i64> {
    let map = aoc::parse_list::<String>(input)?
        .iter()
        .map(|x| x.chars().collect_vec())
        .to_owned()
        .collect_vec();

    // Find starting position
    let mut start_pos = (0, 0);
    'START_SEARCH: for (j, row) in map.iter().enumerate() {
        for (i, cell) in map[j].iter().enumerate() {
            if row[i] == 'S' {
                start_pos = (i, j);
                break 'START_SEARCH;
            }
        }
    }

    println!("Start at {:?}", start_pos);

    let answer = [
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

    let answer = answer as i64 / 2;

    Ok(answer)
}

fn get_path_size(
    start_pos: &(usize, usize, Direction),
    start_char: &char,
    map: &[Vec<char>],
) -> Option<usize> {
    let mut steps = 0;
    let mut pos = start_pos.to_owned();

    let mut map = map.to_vec();
    map[start_pos.1][start_pos.0] = *start_char;

    println!("Starting at {:?}, {start_char}", pos);
    while let Some(new_pos) = next_pos(&pos, &map) {
        pos = new_pos;
        steps += 1;
        println!("At {:?} ", pos);

        if pos.0 == start_pos.0 && pos.1 == start_pos.1 && steps > 0 {
            return Some(steps);
        }
    }

    None
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
        // ('.', _) => None,
        _ => None,
        // _ => panic!(
        //     "Unexpected map character {s_char} and direction {:?}",
        //     start_pos.2
        // ),
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
    // diff.map(|next| (start_pos.0 + next.0, start_pos.1 + next.1, next.2))
    //     .filter(|pos| map.get(pos.1).and_then(|row| row.get(pos.0)).is_some());
}

fn tests() -> anyhow::Result<()> {
    let input = ".....
.S-7.
.|.|.
.L-J.
.....
";

    let solution = solve(input)?;

    assert_eq!(solution, 4);

    let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...
";

    let solution = solve(input)?;

    assert_eq!(solution, 8);

    Ok(())
}
