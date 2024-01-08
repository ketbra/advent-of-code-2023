use anyhow::Result;
use itertools::Itertools;
use pathfinding::prelude::astar;
use pathfinding::prelude::dijkstra_all;
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
    steps_in_same_direction: usize,
}

impl Pos {
    fn neighbors(&self, map: &Vec<Vec<u32>>) -> Vec<(Pos, u32)> {
        let mut neighbors: Vec<Pos> = Vec::new();
        // let mut points_to_consider: Vec<Pos> = Vec::new();

        // Straight
        if self.steps_in_same_direction < 10 {
            match self.direction {
                North => {
                    if self.row > 0 {
                        neighbors.push(Pos {
                            col: self.col,
                            row: self.row - 1,
                            direction: self.direction,
                            steps_in_same_direction: self.steps_in_same_direction + 1,
                        })
                    }
                }
                South => {
                    if self.row < map.len() - 1 {
                        neighbors.push(Pos {
                            col: self.col,
                            row: self.row + 1,
                            direction: self.direction,
                            steps_in_same_direction: self.steps_in_same_direction + 1,
                        })
                    }
                }
                East => {
                    if self.col < map[0].len() - 1 {
                        neighbors.push(Pos {
                            col: self.col + 1,
                            row: self.row,
                            direction: self.direction,
                            steps_in_same_direction: self.steps_in_same_direction + 1,
                        })
                    }
                }
                West => {
                    if self.col > 0 {
                        neighbors.push(Pos {
                            col: self.col - 1,
                            row: self.row,
                            direction: self.direction,
                            steps_in_same_direction: self.steps_in_same_direction + 1,
                        })
                    }
                }
            };
        }

        if self.steps_in_same_direction > 3 {
            match self.direction {
                North => {
                    if self.col > 0 {
                        neighbors.push(Pos {
                            col: self.col - 1,
                            row: self.row,
                            direction: West,
                            steps_in_same_direction: 1,
                        })
                    }
                }
                South => {
                    if self.col < map[0].len() - 1 {
                        neighbors.push(Pos {
                            col: self.col + 1,
                            row: self.row,
                            direction: East,
                            steps_in_same_direction: 1,
                        })
                    }
                }
                East => {
                    if self.row > 0 {
                        neighbors.push(Pos {
                            col: self.col,
                            row: self.row - 1,
                            direction: North,
                            steps_in_same_direction: 1,
                        })
                    }
                }
                West => {
                    if self.row < map.len() - 1 {
                        neighbors.push(Pos {
                            col: self.col,
                            row: self.row + 1,
                            direction: South,
                            steps_in_same_direction: 1,
                        })
                    }
                }
            };

            // Right
            match self.direction {
                North => {
                    if self.col < map[0].len() - 1 {
                        neighbors.push(Pos {
                            col: self.col + 1,
                            row: self.row,
                            direction: East,
                            steps_in_same_direction: 1,
                        })
                    }
                }
                South => {
                    if self.col > 0 {
                        neighbors.push(Pos {
                            col: self.col - 1,
                            row: self.row,
                            direction: West,
                            steps_in_same_direction: 1,
                        })
                    }
                }
                East => {
                    if self.row < map.len() - 1 {
                        neighbors.push(Pos {
                            col: self.col,
                            row: self.row + 1,
                            direction: South,
                            steps_in_same_direction: 1,
                        })
                    }
                }
                West => {
                    if self.row > 0 {
                        neighbors.push(Pos {
                            col: self.col,
                            row: self.row - 1,
                            direction: North,
                            steps_in_same_direction: 1,
                        })
                    }
                }
            };
        }

        let neighbors = neighbors
            .iter()
            .map(|n| (n.to_owned(), map[n.row][n.col]))
            .collect_vec();

        neighbors
    }
}

#[aoc::main]
fn solve(input: &str) -> Result<u32> {
    let map = aoc::parse_list::<String>(input)?
        .iter()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();

    // Create a heuristic ditance map indicating the best case distance of a cell from the end.
    // We create the heuristic using dijkstra based on the problem without the turning restrictions.
    // Creating this better heuristic first turns out to be worth the effort and saves time in the overall
    // process.
    let heuristic_map = get_heuristic_map(&map);

    let starts = vec![
        Pos {
            row: 0,
            col: 0,
            direction: East,
            steps_in_same_direction: 1,
        },
        Pos {
            row: 0,
            col: 0,
            direction: South,
            steps_in_same_direction: 1,
        },
    ];

    let end = Pos {
        row: map.len() - 1,
        col: map[0].len() - 1,
        direction: East,            // Ignored in goal test
        steps_in_same_direction: 1, // Ignored in goal test
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
                |p| p.row == end.row && p.col == end.col && p.steps_in_same_direction >= 4,
            )
            .map(|found| found.1)
            .unwrap()
        })
        .min()
        .unwrap();

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
                neighbors.push(((nc, nr), map[nr][nc].clone() as usize));
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
            }
            else {
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
