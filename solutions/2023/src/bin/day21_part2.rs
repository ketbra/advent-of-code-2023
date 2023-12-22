use anyhow::Result;
use itertools::Itertools;
use lazy_regex::regex_captures;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use Corner::*;
use Parity::*;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Pos {
    row: usize,
    col: usize,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
enum Corner {
    NW,
    NE,
    SW,
    SE,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
enum Parity {
    Even,
    Odd,
}

#[aoc::main]
fn solve(input: &str) -> Result<usize> {
    // part2(input, 26501365)
    part2(input, 5)
}

fn part2(input: &str, max_steps: usize) -> Result<usize> {
    let mut map = aoc::parse_list::<String>(input)?
        .iter()
        .map(|x| x.chars().collect_vec())
        .to_owned()
        .collect_vec();

    // Get the start position, and replace it with a garden plot
    let mut start_pos = None;
    let height = map.len();
    let width = map[0].len();
    for j in 0..height {
        for i in 0..width {
            if map[j][i] == 'S' {
                start_pos = Some(Pos { row: j, col: i });
                // Can't step on start since steps are odd
                map[j][i] = '.';
                break;
            }
        }
    }

    let start_pos = start_pos.unwrap();

    // let marked_map = mark_reachable(&map, &start_pos, max_steps);
    // // print_map(&marked_map);

    // let answer = count_marked_plots(&marked_map);

    let max_fill = [
        get_max_fill_count(&map, Even),
        get_max_fill_count(&map, Odd),
    ];

    println!("{max_fill:?}");

    // Determine minimum steps to fill whole grid from each corner and
    // step modularity.  We'll need this for counting later
    let fill_counts = determine_min_fill_counts(&map, &max_fill);

    println!("{fill_counts:?}");

    let answer = count_reachable_infinite(&map, &start_pos, max_steps, &fill_counts, &max_fill);

    Ok(answer)
}

fn get_max_fill_count(map: &[Vec<char>], parity: Parity) -> usize {
    let mut count = 0;
    // Count squares with same parity and no rock
    for (j, row) in map.iter().enumerate() {
        for (i, c) in row.iter().enumerate() {
            if *c != '#' {
                count += match (((i + j) % 2) == 0, &parity) {
                    (true, Parity::Even) | (false, Parity::Odd) => 1,
                    (false, Parity::Even) | (true, Parity::Odd) => 0,
                }
            }
        }
    }
    count
}

fn determine_min_fill_counts(
    map: &[Vec<char>],
    max_fill: &[usize],
) -> HashMap<(Corner, Parity), usize> {
    let mut fill_counts = HashMap::new();
    let height = map.len();
    let width = map[0].len();

    for corner in [NW, NE, SW, SE] {
        let starting_pos = match corner {
            NW => Pos { row: 0, col: 0 },
            NE => Pos {
                row: 0,
                col: width - 1,
            },
            SW => Pos {
                row: height - 1,
                col: 0,
            },
            SE => Pos {
                row: height - 1,
                col: width - 1,
            },
        };

        for parity in [Even, Odd] {
            let mut high = 50000;
            let mut low = 1;

            let target = match parity {
                Even => max_fill[0],
                Odd => max_fill[1],
            };
            while high != low && high - low > 1 {
                // Pick mid point
                let mid = (high + low) / 2;
                let steps = match parity {
                    Even => 2 * mid,
                    Odd => 2 * mid - 1,
                };

                let count = count_reachable(map, &starting_pos, steps);
                if count >= target {
                    high = mid;
                } else {
                    low = mid;
                }
            }
            let steps = match parity {
                Even => 2 * low,
                Odd => 2 * low - 1,
            };
            // println!("{starting_pos:?}, {parity:?}, {low}, {high}.  Steps={steps}");
            fill_counts.insert((corner.clone(), parity), steps);
            // let mut max = fill_counts.insert((corner.clone(), parity), 10);
        }
    }

    fill_counts
}

fn count_reachable_infinite(
    map: &[Vec<char>],
    start: &Pos,
    max_steps: usize,
    fill_counts: &HashMap<(Corner, Parity), usize>,
    max_fill: &[usize],
) -> usize {
    let mut total = 0;
    let height = map.len();
    let width = map[0].len();

    // let mut steps = (width - 1)/2;
    // Add the starting tile where we start in the middle

    // Add middle column
    let mut steps_left = max_steps as isize;

    // Count the middle title
    let distance_to_corner = 11; // Test only
    total += count_reachable(map, start, steps_left as usize);
    steps_left -= distance_to_corner;

    // Get minimum fill count
    let mut min_fill_count = *fill_counts.get(&(NW, Even)).unwrap();
    for corner in [NW, SW, NE, SE] {
        for parity in [Even, Odd] {
            min_fill_count =
                min_fill_count.max(*fill_counts.get(&(corner.clone(), parity)).unwrap());
        }
    }

    let nw_point = Pos { col: 0, row: 0 };
    let ne_point = Pos {
        col: width - 1,
        row: 0,
    };
    let sw_point = Pos {
        col: 0,
        row: height - 1,
    };
    let se_point = Pos {
        col: width - 1,
        row: height - 1,
    };

    let starts_up = [&se_point, &sw_point];
    let starts_down = [&ne_point, &nw_point];

    while steps_left > 0 {
        if steps_left as usize > min_fill_count {
            total += 2 * max_fill[steps_left as usize % 2]; // Up and down
        } else {
            total += count_reachable_multiple(map, &starts_up, steps_left as usize);
            total += count_reachable_multiple(map, &starts_down, steps_left as usize);
        }
        steps_left -= height as isize;
    }

    // dbg!(max_steps);
    // dbg!(width);

    let distance_to_next_title = ((width - 1) / 2) + ((height - 1) / 2) + 1;

    // Now move up to the bottom left of the above tile and count upwards
    if max_steps > distance_to_next_title {
        total += count_vertical(
            map,
            &Pos {
                row: height - 1,
                col: 0,
            },
            max_steps,
            fill_counts,
            max_fill,
        );
        total += count_vertical(
            map,
            &Pos { row: 0, col: 0 },
            max_steps,
            fill_counts,
            max_fill,
        );
    }

    // Count columns to left and right

    // test only
    let distance_to_corner = 11;

    let corners = [
        Pos { row: 0, col: 0 },
        Pos {
            row: 0,
            col: (width - 1),
        },
        Pos {
            row: (height - 1),
            col: 0,
        },
        Pos {
            row: (height - 1),
            col: (width - 1),
        },
    ];

    if max_steps > distance_to_corner {
        let mut steps_left = max_steps - distance_to_corner;

        loop {
            println!("There are {steps_left} steps left");

            for corner_pos in &corners {
                total += count_vertical(map, corner_pos, steps_left, fill_counts, max_fill);
            }

            if steps_left > width {
                steps_left -= width;
            } else {
                break;
            }
        }
    }

    // let mut steps_left = max_steps;
    // if steps_left > distance_to_next_title {
    //     steps_left -= distance_to_next_title;
    //     total += count_reachable(
    //         map,
    //         &Pos {
    //             row: start.row,
    //             col: (width - 1),
    //         },
    //         steps_left,
    //     );

    //     total += count_reachable(
    //         map,
    //         &Pos {
    //             row: start.row,
    //             col: 0,
    //         },
    //         steps_left,
    //     );
    // }

    // Count tile to right

    // // println!("{max_steps} - half {width} = {steps_left}");
    // while steps_left > width {
    //     total += count_reachable_infinite_column(map, start, steps_left, max_fill);
    //     steps_left -= width;
    //     println!("{steps_left}");
    // }

    total
}

fn start_pos_to_corner(map: &[Vec<char>], starting_pos: &Pos) -> Corner {
    let height = map.len();
    let width = map[0].len();

    match starting_pos {
        Pos { col: 0, row: 0 } => NW,
        Pos { col: c, row: 0 } if *c == width - 1 => NE,
        Pos { col: 0, row: r } if *r == height - 1 => SW,
        Pos { col: c, row: r } if *c == width - 1 && *r == height - 1 => SE,
        x => panic!("Unexpected starting position, {x:?}"),
    }
}

fn count_vertical(
    map: &[Vec<char>],
    starting_pos: &Pos,
    steps_left: usize,
    fill_counts: &HashMap<(Corner, Parity), usize>,
    max_fill: &[usize],
) -> usize {
    let height = map.len();
    let mut total = 0;

    let mut steps_left = steps_left;
    let starting_corner = start_pos_to_corner(map, starting_pos);

    // println!("Counting upwards with {steps_left} steps left");

    while steps_left > height {
        let modulus = steps_left % 2;
        let parity = match modulus == 0 {
            true => Even,
            false => Odd,
        };

        if steps_left > fill_counts[&(starting_corner.clone(), parity)] {
            total += max_fill[modulus];
        } else {
            total += count_reachable(map, starting_pos, steps_left);
        }

        steps_left -= height;
    }

    total
}

fn count_reachable_infinite_column(
    map: &[Vec<char>],
    start: &Pos,
    max_steps: usize,
    max_fill: &[usize],
) -> usize {
    let height = map.len();
    let width = map[0].len();

    // First, count whatever you can reach in the current square
    let mut total = count_reachable(map, start, max_steps);
    let mut steps_left = max_steps;

    total
}

fn print_map(map: &[Vec<char>]) {
    for row in map {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}

fn count_reachable(map: &[Vec<char>], start: &Pos, max_steps: usize) -> usize {
    let marked_map = mark_reachable(map, start, max_steps);
    // print_map(&marked_map);
    // println!();
    count_marked_plots(&marked_map)
}

fn count_reachable_multiple(map: &[Vec<char>], start: &[&Pos], max_steps: usize) -> usize {
    let mut starts = start.to_vec();
    let mut marked_map = mark_reachable(map, starts.pop().unwrap(), max_steps);

    while let Some(pos) = starts.pop() {
        marked_map = mark_reachable(&marked_map, pos, max_steps);
    }
    // print_map(&marked_map);
    // println!();
    count_marked_plots(&marked_map)
}

fn mark_reachable(map: &[Vec<char>], start: &Pos, max_steps: usize) -> Vec<Vec<char>> {
    let mut new_map = map.to_vec();
    let mark_mod = max_steps % 2;

    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();

    let height = map.len();
    let width = map[0].len();

    let mut pos = start;
    // seen.insert((0, pos.clone()));
    queue.push_back((0, pos.clone()));

    let mut work = 0;
    while let Some((steps, pos)) = queue.pop_front() {
        work += 1;
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
                    let new_state = (new_steps % 2, new_pos.clone());
                    if !seen.contains(&new_state) {
                        seen.insert(new_state);
                        if map[new_row][new_col] != '#' {
                            if new_steps % 2 == mark_mod {
                                new_map[new_row][new_col] = 'O';
                            }
                            if new_steps != max_steps {
                                queue.push_back((new_steps, new_pos));
                                // new_map[new_row][new_col] = 'O';
                            }
                        }
                    }
                }
            }
        }
    }

    // println!("Work done={work}");
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

    let solution = part2(input, 6)?;
    assert_eq!(solution, 16);
    // let solution = part2(input, 10)?;
    // assert_eq!(solution, 50);

    // let solution = part2(input, 50)?;
    // assert_eq!(solution, 1594);

    // let solution = part2(input, 100)?;
    // assert_eq!(solution, 6536);

    // let solution = part2(input, 26501365)?;
    // assert_eq!(solution, 16733044);

    let solution = part2(input, 500)?;
    assert_eq!(solution, 167004);

    let solution = part2(input, 1000)?;
    assert_eq!(solution, 668697);

    let solution = part2(input, 5000)?;
    assert_eq!(solution, 16733044);

    Ok(())
}
