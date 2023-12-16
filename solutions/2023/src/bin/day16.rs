use anyhow::Result;
use itertools::Itertools;
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

#[derive(Debug, Clone, PartialEq, Copy)]
struct Position {
    row: usize,
    col: usize,
}

#[derive(Debug, Clone, PartialEq, Copy)]
struct LightRay {
    direction: Direction,
}

#[derive(Debug, Clone, PartialEq)]
struct Tile {
    c: char,
    light_rays: Vec<LightRay>,
    unfollowed_rays: Vec<LightRay>,
}

#[aoc::main]
fn solve(input: &str) -> Result<usize> {
    let mut map = aoc::parse_list::<String>(input)?
        .iter()
        .map(|x| {
            x.chars()
                .map(|c| Tile {
                    c,
                    light_rays: Vec::new(),
                    unfollowed_rays: Vec::new(),
                })
                .collect_vec()
        })
        .to_owned()
        .collect_vec();

    let answer = calculate_energized_tiles(&map);

    Ok(answer)
}

fn calculate_energized_tiles(map: &[Vec<Tile>]) -> usize {
    let width = map[0].len();
    let height = map.len();

    let mut map = map.to_vec();
    energize_tiles(&mut map);

    let mut energized = 0;
    for row in map {
        for tile in row {
            if !tile.light_rays.is_empty() {
                energized += 1;
            }
        }
    }
    energized
}

fn energize_tiles(map: &mut [Vec<Tile>]) {
    let width = map[0].len();
    let height = map.len();

    let mut queue = VecDeque::new();
    map[0][0].light_rays.push(LightRay { direction: East });
    map[0][0].unfollowed_rays.push(LightRay { direction: East });
    queue.push_back(Position { row: 0, col: 0 });

    // Don't follow rays we've already seen

    // let seen: HashSet<(usize, usize, Direction)> = HashSet::new();
    let mut seen = HashSet::new();

    while let Some(pos) = queue.pop_front() {
        let tile = &map[pos.row][pos.col].clone();
        for ray in &tile.unfollowed_rays {
            let neighbors = match (ray.direction, tile.c) {
                (West | East, '|') => vec![North, South],
                (dir, '|') => vec![dir],
                (North | South, '-') => vec![West, East],
                (dir, '-') => vec![dir],
                (North, '\\') => vec![West],
                (South, '\\') => vec![East],
                (East, '\\') => vec![South],
                (West, '\\') => vec![North],
                (North, '/') => vec![East],
                (South, '/') => vec![West],
                (East, '/') => vec![North],
                (West, '/') => vec![South],
                (dir, '.') => vec![dir],
                _ => panic!("Missing a direction/char combination"),
            };

            for direction in neighbors {
                let neighbor = match direction {
                    North => {
                        if pos.row > 0 {
                            Some(Position {
                                row: pos.row - 1,
                                col: pos.col,
                            })
                        } else {
                            None
                        }
                    }
                    South => {
                        if pos.row < height - 1 {
                            Some(Position {
                                row: pos.row + 1,
                                col: pos.col,
                            })
                        } else {
                            None
                        }
                    }
                    East => {
                        if pos.col < width - 1 {
                            Some(Position {
                                row: pos.row,
                                col: pos.col + 1,
                            })
                        } else {
                            None
                        }
                    }
                    West => {
                        if pos.col > 0 {
                            Some(Position {
                                row: pos.row,
                                col: pos.col - 1,
                            })
                        } else {
                            None
                        }
                    }
                };

                if let Some(neighbor) = neighbor {
                    if !seen.contains(&(neighbor.col, neighbor.row, direction)) {
                        map[neighbor.row][neighbor.col]
                            .unfollowed_rays
                            .push(LightRay { direction });
                        map[neighbor.row][neighbor.col]
                            .light_rays
                            .push(LightRay { direction });
                        queue.push_back(neighbor);
                        seen.insert((neighbor.col, neighbor.row, direction));
                    }
                }
            }
        }
        // Clear out the unfollowed ray list we just considered
        map[pos.row][pos.col].unfollowed_rays.clear();
    }
}

fn tests() -> anyhow::Result<()> {
    let input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
";

    let solution = solve(input)?;

    assert_eq!(solution, 46);

    Ok(())
}
