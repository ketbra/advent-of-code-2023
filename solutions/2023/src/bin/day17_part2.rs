use anyhow::Result;
use itertools::Itertools;
use pathfinding::prelude::astar;
use pathfinding::prelude::dijkstra_all;
use std::time::Instant;
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
    row: usize,
    col: usize,
    direction: Direction,
}

impl Pos {
    fn neighbors(&self, map: &Vec<Vec<u32>>) -> Vec<(Pos, u32)> {
        let mut neighbors: Vec<(Pos, u32)> = Vec::new();

        let moves = match self.direction {
            East | West => [
                (
                    North,
                    [
                        (0, -1, false),
                        (0, -2, false),
                        (0, -3, false),
                        (0, -4, true),
                        (0, -5, true),
                        (0, -6, true),
                        (0, -7, true),
                        (0, -8, true),
                        (0, -9, true),
                        (0, -10, true),
                    ],
                ),
                (
                    South,
                    [
                        (0, 1, false),
                        (0, 2, false),
                        (0, 3, false),
                        (0, 4, true),
                        (0, 5, true),
                        (0, 6, true),
                        (0, 7, true),
                        (0, 8, true),
                        (0, 9, true),
                        (0, 10, true),
                    ],
                ),
            ],
            North | South => [
                (
                    West,
                    [
                        (-1, 0, false),
                        (-2, 0, false),
                        (-3, 0, false),
                        (-4, 0, true),
                        (-5, 0, true),
                        (-6, 0, true),
                        (-7, 0, true),
                        (-8, 0, true),
                        (-9, 0, true),
                        (-10, 0, true),
                    ],
                ),
                (
                    East,
                    [
                        (1, 0, false),
                        (2, 0, false),
                        (3, 0, false),
                        (4, 0, true),
                        (5, 0, true),
                        (6, 0, true),
                        (7, 0, true),
                        (8, 0, true),
                        (9, 0, true),
                        (10, 0, true),
                    ],
                ),
            ],
        };

        let rows = map.len();
        let cols = map[0].len();

        for (dir, deltas) in moves {
            let mut cost = 0;
            for (dc, dr, add) in deltas {
                let (nc, nr) = (self.col as isize + dc, self.row as isize + dr);
                if nc >= 0 && nr >= 0 && nr < rows as isize && nc < cols as isize {
                    let (nc, nr) = (nc as usize, nr as usize);
                    cost += map[nr][nc];
                    if add {
                        neighbors.push((
                            Pos {
                                col: nc,
                                row: nr,
                                direction: dir,
                            },
                            cost,
                        ));
                    }
                }
            }
        }

        neighbors
    }
}

#[aoc::main]
fn solve(input: &str) -> Result<u32> {
    let now = Instant::now();
    let map = aoc::parse_list::<String>(input)?
        .iter()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();

    // Create a heuristic ditance map indicating the best case distance of a cell from the end.
    // We create the heuristic using dijkstra based on the problem without the turning restrictions.
    // Creating this better heuristic first turns out to be worth the effort and saves time in the overall
    // process.
    let heuristic_map_start = Instant::now();
    let heuristic_map = get_heuristic_map(&map);
    let heuristic_map_elapsed = heuristic_map_start.elapsed().as_millis();

    // There's a subtlety here.  The start going east will immediately
    // add nodes going south from neighbors and vice versa.  This is
    // awkward, but it ends up working out properly.
    let starts = vec![
        Pos {
            row: 0,
            col: 0,
            direction: East,
        },
        Pos {
            row: 0,
            col: 0,
            direction: South,
        },
    ];

    let end = Pos {
        row: map.len() - 1,
        col: map[0].len() - 1,
        direction: East, // Ignored in goal test
    };

    let answer = starts
        .iter()
        .map(|start| {
            astar(
                start,
                |p| p.neighbors(&map),
                |p| {
                    if p.col == end.col && p.row == end.row {
                        return 0;
                    }

                    // heuristic_map.get(&(p.col, p.row)).unwrap().1 as u32
                    heuristic_map[p.row][p.col]
                },
                |p| p.row == end.row && p.col == end.col,
            )
            .map(|found| found.1)
            .unwrap()
        })
        .min()
        .unwrap();

    let elapsed = now.elapsed().as_millis();
    println!("Generated heuristic map in {heuristic_map_elapsed} ms");
    println!("Solve took {elapsed} ms");
    Ok(answer)
}

fn get_heuristic_map(map: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let rows = map.len();
    let cols = map[0].len();

    let successors = |n: &(usize, usize)| -> Vec<((usize, usize), usize)> {
        let (c, r) = n;
        let mut neighbors = vec![];
        for (dc, dr) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let (nc, nr) = (*c as isize + dc, *r as isize + dr);
            if nc >= 0 && nr >= 0 && nr < rows as isize && nc < cols as isize {
                let (nc, nr) = (nc as usize, nr as usize);
                neighbors.push(((nc, nr), map[nr][nc] as usize));
            }
        }

        neighbors
    };

    let d = dijkstra_all(&(cols - 1, rows - 1), successors);

    let mut hmap = map.to_vec();
    for (j, row) in hmap.iter_mut().enumerate() {
        for (i, v) in row.iter_mut().enumerate() {
            if i == cols - 1 && j == rows - 1 {
                *v = 0;
            } else {
                *v = d.get(&(i, j)).unwrap().1 as u32;
            }
        }
    }
    hmap
}

fn tests() -> anyhow::Result<()> {
    let input = r"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";

    let solution = solve(input)?;

    assert_eq!(solution, 94);

    let input = r"111111111111
999999999991
999999999991
999999999991
999999999991
";

    let solution = solve(input)?;

    assert_eq!(solution, 71);

    Ok(())
}
