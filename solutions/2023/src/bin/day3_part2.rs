use anyhow::Result;
use aoc::Searchable;
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
struct NumNode {
    id: usize,
    val: u64,
}

#[aoc::main]
fn solve(input: &str) -> Result<u64> {
    let vals: Vec<String> = aoc::parse_list(input)?;

    // println!("{:?}", vals);

    // Split up the lines into chars
    let lines: Vec<Vec<char>> = vals.iter().map(|line| line.chars().collect_vec()).collect();
    let mut answer = 0;
    let mut id_counter: usize = 0;
    let mut number_map = HashMap::new();

    // Build the map of numbers found at each location
    (0..lines.len()).for_each(|i| {
        let line = &lines[i];
        let mut val = "".to_string();
        let mut start = 0;
        (0..line.len()).for_each(|j| {
            let c = line[j];
            id_counter += 1;
            let id = id_counter;
            if c.is_ascii_digit() {
                val.push(c);

                if j == line.len() - 1 && !val.is_empty() {
                    let val = val.parse::<u64>().unwrap();
                    let node = NumNode { id, val };
                    for x in start..=j {
                        number_map.insert((x, i), node);
                    }
                }
            } else {
                if !val.is_empty() {
                    let val = val.parse::<u64>().unwrap();
                    let node = NumNode { id, val };
                    for x in start..j {
                        number_map.insert((x, i), node);
                    }
                }
                start = j + 1;
                val = "".to_string();
            }
        })
    });

    // println!("{:?}", number_map);

    (0..lines.len()).for_each(|i| {
        let line = &lines[i];
        (0..line.len()).for_each(|j| {
            if line[j] == '*' {
                let mut adjacent_nums = HashMap::new();
                neighbors(j, i).iter().for_each(|(x, y)| {
                    if let Some(node) = number_map.get(&(*x, *y)) {
                        // println!("Found adjacent node at {}, {}", x, y);
                        adjacent_nums.insert(node.id, node.val);
                    }
                });
                if adjacent_nums.len() == 2 {
                    let nums: Vec<_> = adjacent_nums.values().collect();

                    // println!("Gear at {},{} is next to {} and {}", j, i, nums[0], nums[1]);
                    answer += nums[0] * nums[1];
                } else {
                    println!("No ratio for gear at {}, {}", j, i);
                    println!("{:?}", adjacent_nums);
                    println!("{:?}", &lines[i - 1][j - 1..=j + 1]);
                    println!("{:?}", &lines[i][j - 1..=j + 1]);
                    println!("{:?}", &lines[i + 1][j - 1..=j + 1]);
                }
            }
        });
    });
    // println!("{:?}", number_map);
    Ok(answer)
}

fn neighbors(x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut xs = vec![(x, y + 1), (x + 1, y), (x + 1, y + 1)];
    if x > 0 {
        if y > 0 {
            xs.push((x - 1, y - 1));
        }
        xs.push((x - 1, y));
        xs.push((x - 1, y + 1));
    }
    if y > 0 {
        xs.push((x, y - 1));
        xs.push((x + 1, y - 1));
    }
    xs
}

fn tests() -> anyhow::Result<()> {
    let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

    let solution = solve(input)?;

    assert_eq!(solution, 467835);
    Ok(())
}
