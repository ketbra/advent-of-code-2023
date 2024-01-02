use anyhow::Result;
use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Pos {
    row: usize,
    col: usize,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct IPos {
    row: isize,
    col: isize,
}

#[aoc::main]
fn solve(input: &str) -> Result<usize> {
    // let steps = 5000;
    // let expected = part2_slow(input, steps).unwrap();
    // let got = part2(input, steps).unwrap();

    // assert_eq!(got, expected);
    // println!("expected ({expected}) == got ({got})");

    part2(input, 26501365)
}

fn part2(input: &str, max_steps: usize) -> Result<usize> {
    let mut map = aoc::parse_list::<String>(input)?
        .iter()
        .map(|x| x.chars().collect_vec())
        .to_owned()
        .collect_vec();

    // Get the start position, and replace it with a garden plot
    let mut start_pos = None;
    let height = map.len() as isize;
    let width = map[0].len() as isize;
    let max_steps = max_steps as isize;
    for j in 0..height {
        let j = j as usize;
        for i in 0..width {
            let i = i as usize;
            if map[j][i] == 'S' {
                start_pos = Some(Pos { row: j, col: i });
                // Can't step on start since steps are odd
                map[j][i] = '.';
                break;
            }
        }
    }

    // get_reachable_count_above

    // Add left and right columns
    let start_pos = start_pos.unwrap();

    let nw_point = Pos { col: 0, row: 0 };
    let ne_point = Pos {
        col: width as usize - 1,
        row: 0,
    };
    let sw_point = Pos {
        col: 0,
        row: height as usize - 1,
    };
    let se_point = Pos {
        col: width as usize - 1,
        row: height as usize - 1,
    };

    let mut cache = HashMap::new();
    let corner_points = [
        ne_point.clone(),
        nw_point.clone(),
        se_point.clone(),
        sw_point.clone(),
    ];

    // Count all but middle row and column
    println!("Adding 4 corners");
    let mut answer = 0;

    // First move to closest corner of diagonal tile
    let mut steps_left = (max_steps - width - 1) as isize;
    let mut corner_count = 0;
    while steps_left >= 0 {
        // Add the columns for each of the 4 diagonal directions
        for pos in &corner_points {
            corner_count += get_reachable_count_above(&map, pos, steps_left as usize, &mut cache);
        }

        steps_left -= width as isize;
    }
    answer += corner_count;
    println!("Corner count={corner_count}");

    println!("Adding central tile");
    answer += count_reachable(&map, &start_pos, max_steps as usize);

    // Adding middle tiles
    println!("Adding middle tiles");
    let pos_left = Pos {
        row: start_pos.row,
        col: width as usize - 1,
    };
    let pos_right = Pos {
        row: start_pos.row,
        col: 0,
    };
    let pos_up = Pos {
        row: height as usize - 1,
        col: start_pos.col,
    };
    let pos_down = Pos {
        row: 0,
        col: start_pos.col,
    };
    let steps_left = max_steps - ((width - 1) / 2) - 1;
    dbg!(steps_left);
    if steps_left >= 0 {
        for pos in [pos_left, pos_right, pos_up, pos_down] {
            answer += get_reachable_count_above(&map, &pos, steps_left as usize, &mut cache);
        }
    }

    Ok(answer)
}

fn get_reachable_count_above(
    map: &[Vec<char>],
    pos: &Pos,
    steps_left: usize,
    cache: &mut HashMap<(Pos, usize), usize>,
) -> usize {
    let mut cache_key = (pos.clone(), steps_left);
    if let Some(x) = cache.get(&cache_key) {
        return *x;
    }

    let height = map.len();

    let max_steps = steps_left;
    let mut steps_left = steps_left;
    let mut total = 0;
    while steps_left > height {
        cache_key.1 = steps_left;
        if let Some(x) = cache.get(&cache_key) {
            total = *x;
            break;
        }
        steps_left -= height;
    }

    let threshold = 300;
    let reachable_cache = [
        count_reachable(map, pos, threshold),
        count_reachable(map, pos, threshold + 1),
    ];

    println!("{pos:?}, {reachable_cache:?}");

    while steps_left <= max_steps {
        let reachable = if steps_left > threshold {
            reachable_cache[steps_left % 2]
        } else {
            count_reachable(map, pos, steps_left)
        };
        total += reachable;
        cache.insert((pos.clone(), steps_left), total);
        steps_left += height;
    }

    total
}

fn count_reachable(map: &[Vec<char>], start: &Pos, max_steps: usize) -> usize {
    let marked_map = mark_reachable(map, start, max_steps);
    count_marked_plots(&marked_map)
}

fn mark_reachable(map: &[Vec<char>], start: &Pos, max_steps: usize) -> Vec<Vec<char>> {
    let mut new_map = map.to_vec();
    let mark_mod = max_steps % 2;

    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();

    let height = map.len();
    let width = map[0].len();

    // We can always finish on the current square if we're taking an
    // even number of steps
    if mark_mod == 0 {
        new_map[start.row][start.col] = 'O';
    }

    if max_steps > 0 {
        queue.push_back((0, start.clone()));
    }

    while let Some((steps, pos)) = queue.pop_front() {
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
                        if map[new_row][new_col] != '#' || map[new_row][new_col] == 'S' {
                            if new_steps % 2 == mark_mod {
                                new_map[new_row][new_col] = 'O';
                            }
                            if new_steps != max_steps {
                                queue.push_back((new_steps, new_pos));
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
    //     let input = r"...........
    // .....###.#.
    // .###.##..#.
    // ..#.#...#..
    // ....#.#....
    // .##..S####.
    // .##..#...#.
    // .......##..
    // .##.#.####.
    // .##..##.##.
    // ...........
    // ";

    // Doesn't work on the test input since it doesn't have the straight away middle column.  I could adapt for this,
    // but it's not worth it
    //     let solution = part2(input, 5000)?;
    //     assert_eq!(solution, 16733044);

    Ok(())
}
